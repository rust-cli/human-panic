extern crate x11_dl;

use std::ffi::CString;
use std::mem;
use std::os::raw::*;
use std::ptr;

use self::x11_dl::xlib;
use self::x11_dl::xlib::Xlib;
use self::x11_dl::xlib::XRectangle;

const TEXT_MARGIN: i32 = 10;

pub fn create_window(message: String) {
  // window state
  let mut window_width = 600;
  let mut window_height = 400;

  // load Xlib library
  let xlib = match Xlib::open() {
    Ok(xlib) => xlib,
    Err(err) => {
      eprintln!("Failed to open xlib: {}", err);
      return;
    }
  };

  // open display connection.
  let display = unsafe { (xlib.XOpenDisplay)(ptr::null()) };
  if display.is_null() {
    eprintln!("XOpenDisplay failed");
    return;
  }
  // create window.
  let window = unsafe {
    let screen = (xlib.XDefaultScreen)(display);
    let root = (xlib.XRootWindow)(display, screen);
    let mut attributes: xlib::XSetWindowAttributes = mem::uninitialized();
    attributes.background_pixel = (xlib.XWhitePixel)(display, screen);

    // window height gets reset later
    (xlib.XCreateWindow)(display,
                         root,
                         0,
                         0,
                         window_width,
                         window_height,
                         0,
                         0,
                         xlib::InputOutput as c_uint,
                         ptr::null_mut(),
                         xlib::CWBackPixel,
                         &mut attributes)
  };

  // set window title.
  let title_str = CString::new("Panic!").unwrap();
  unsafe {
    (xlib.XStoreName)(display, window, title_str.as_ptr() as *const c_char)
  };

  // allow the window to be deleted by the window manager
  let wm_protocols_str = CString::new("WM_PROTOCOLS").unwrap();
  let wm_delete_window_str = CString::new("WM_DELETE_WINDOW").unwrap();
  let wm_protocols = unsafe {
    (xlib.XInternAtom)(display, wm_protocols_str.as_ptr(), xlib::False)
  };
  let wm_delete_window = unsafe {
    (xlib.XInternAtom)(display, wm_delete_window_str.as_ptr(), xlib::False)
  };
  let mut protocols = [wm_delete_window];
  unsafe {
    (xlib.XSetWMProtocols)(display,
                           window,
                           protocols.as_mut_ptr(),
                           protocols.len() as c_int)
  };

  // let the window manager know this is a dialog box.
  let wm_window_type_str = CString::new("_NET_WM_WINDOW_TYPE").unwrap();
  let wm_window_type_dialog_str = CString::new("_NET_WM_WINDOW_TYPE_DIALOG")
    .unwrap();
  let wm_window_type = unsafe {
    (xlib.XInternAtom)(display, wm_window_type_str.as_ptr(), xlib::False)
  };
  let wm_window_type_dialog =
    unsafe {
      (xlib.XInternAtom)(display,
                         wm_window_type_dialog_str.as_ptr(),
                         xlib::False)
    };
  let wm_window_type_dialog = &wm_window_type_dialog as *const u64 as *const u8;
  unsafe {
    (xlib.XChangeProperty)(display,
                           window,
                           wm_window_type,
                           xlib::XA_ATOM,
                           32,
                           xlib::PropModeReplace,
                           wm_window_type_dialog,
                           1)
  };

  // specify events to use
  unsafe {
    (xlib.XSelectInput)(display,
                        window,
                        xlib::ExposureMask | xlib::StructureNotifyMask)
  };

  // create graphics context
  let gc = unsafe {
    let mut values: xlib::XGCValues = mem::uninitialized();
    (xlib.XCreateGC)(display, window, 0, &mut values)
  };

  // create font set
  let font_list = CString::new("-*-*-medium-r-normal--*-120-*-*-*-*-*-*")
    .unwrap();
  let mut missing = ptr::null_mut();
  let mut num_missing = 0;
  let mut foo = ptr::null_mut();
  let font_set = unsafe {
    (xlib.XCreateFontSet)(display,
                          font_list.as_ptr() as *const c_char,
                          &mut missing,
                          &mut num_missing,
                          &mut foo)
  };

  // show window.
  unsafe { (xlib.XMapWindow)(display, window) };

  // determine maximum line height
  let max_line_height = if let Ok(message) = CString::new(message.clone()) {
    line_width_height(&xlib, font_set, &message).1 as i32 + 2
  } else {
    15
  };

  // split message into multiple lines
  let mut message_lines =
    split_message(&xlib, font_set, &message, window_width as i32);

  // resize window to fit text
  window_height = (message_lines.len() as u32 + 1) * max_line_height as u32;
  unsafe { (xlib.XResizeWindow)(display, window, window_width, window_height) };

  loop {
    // wait for next event
    let event = unsafe {
      let mut event: xlib::XEvent = mem::uninitialized();
      (xlib.XNextEvent)(display, &mut event);
      event
    };

    // process event, close if asked to
    match event.get_type() {
      xlib::ClientMessage => {
        let xclient = xlib::XClientMessageEvent::from(event);
        if xclient.message_type == wm_protocols && xclient.format == 32 {
          let protocol = xclient.data.get_long(0) as xlib::Atom;

          if protocol == wm_delete_window {
            break;
          }
        }
      }
      xlib::ConfigureNotify => {
        let configure_event: &xlib::XConfigureEvent = event.as_ref();
        window_width = configure_event.width as u32;
                #[allow(unused_assignments)]
        {
          window_height = configure_event.height as u32;
        }
      }
      xlib::Expose => {
        // draw the window
        message_lines =
          split_message(&xlib, font_set, &message, window_width as i32);
        for (i, line) in message_lines.iter().enumerate() {
          unsafe {
            (xlib.Xutf8DrawString)(display,
                                   window,
                                   font_set,
                                   gc,
                                   TEXT_MARGIN,
                                   (i as i32 + 1) * max_line_height,
                                   line.as_ptr() as *const c_char,
                                   line.to_bytes().len() as i32)
          };
        }
      }
      _ => (),
    }
  }

  // shut down.
  unsafe { (xlib.XCloseDisplay)(display) };
}

fn line_width_height(xlib: &Xlib,
                     font_set: *mut xlib::_XOC,
                     text: &CString)
                     -> (u16, u16) {
  let mut overall_ink = XRectangle {
    x: 0,
    y: 0,
    width: 0,
    height: 0,
  };
  let mut overall_logical = overall_ink.clone();
  unsafe {
    (xlib.Xutf8TextExtents)(font_set,
                            text.as_ptr() as *const c_char,
                            text.to_bytes().len() as i32,
                            &mut overall_ink as *mut XRectangle,
                            &mut overall_logical as *mut XRectangle)
  };

  (overall_logical.width, overall_logical.height)
}

fn split_message(xlib: &Xlib,
                 font_set: *mut xlib::_XOC,
                 message: &String,
                 window_width: i32)
                 -> Vec<CString> {
  let mut processed_lines = vec![];
  for line in message.lines() {
    if line.is_empty() {
      // Just add an empty line
      processed_lines.push(CString::new("").unwrap());
    } else {
      // further split the line
      let mut current_line = String::new();
      let mut new_line = String::new();
      for word in line.split_whitespace() {
        new_line.push_str(word);
        new_line.push(' ');
        if let Ok(line_cstring) = CString::new(new_line.clone()) {
          let (line_width, _) =
            line_width_height(&xlib, font_set, &line_cstring);
          if TEXT_MARGIN + line_width as i32 >= window_width {
            // the new line is too long!
            // add the current line to the processed lines
            // and reset current and new line
            if let Ok(line_cstring) = CString::new(current_line.clone()) {
              processed_lines.push(line_cstring);
            }
            current_line.clear();
            new_line.clear();
            current_line.push_str(word);
            current_line.push(' ');
            new_line.push_str(word);
            new_line.push(' ');
          } else {
            // the new line is fine, update the current line
            current_line = new_line.clone();
          }
        } else {
          // bad data, so clear the line, so we we dont hit it again
          current_line.clear();
          new_line.clear();
        }
      }
      // There are no more words to process add the remaining line to the processed lines
      if !current_line.is_empty() {
        if let Ok(line_cstring) = CString::new(current_line) {
          processed_lines.push(line_cstring);
        }
      }
    }
  }
  processed_lines
}
