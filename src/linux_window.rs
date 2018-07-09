extern crate x11_dl;

use std::ffi::CString;
use std::mem;
use std::os::raw::*;
use std::ptr;

use self::x11_dl::xlib;
use self::x11_dl::xlib::XRectangle;

pub fn create_window(message: String) {
    // load Xlib library
    let xlib = match xlib::Xlib::open() {
        Ok(xlib) => xlib,
        Err(err) => {
            eprintln!("Failed to open xlib: {}", err);
            return
        }
    };

    // open display connection.
    let display = unsafe { (xlib.XOpenDisplay)(ptr::null()) };
    if display.is_null() {
        eprintln!("XOpenDisplay failed");
        return
    }

    // create window.
    let window = unsafe {
        let screen = (xlib.XDefaultScreen)(display);
        let root = (xlib.XRootWindow)(display, screen);
        let mut attributes: xlib::XSetWindowAttributes = mem::uninitialized();
        attributes.background_pixel = (xlib.XWhitePixel)(display, screen);

        (xlib.XCreateWindow)(display, root, 0, 0, 1000, 300, 0, 0,
                             xlib::InputOutput as c_uint, ptr::null_mut(),
                             xlib::CWBackPixel, &mut attributes)
    };

    // set window title.
    let title_str = CString::new("Panic!").unwrap();
    unsafe { (xlib.XStoreName)(display, window, title_str.as_ptr() as *const c_char) };

    // allow the window to be deleted by the window manager
    let wm_protocols_str = CString::new("WM_PROTOCOLS").unwrap();
    let wm_delete_window_str = CString::new("WM_DELETE_WINDOW").unwrap();
    let wm_protocols = unsafe { (xlib.XInternAtom)(display, wm_protocols_str.as_ptr(), xlib::False) };
    let wm_delete_window = unsafe { (xlib.XInternAtom)(display, wm_delete_window_str.as_ptr(), xlib::False) };
    let mut protocols = [wm_delete_window];
    unsafe { (xlib.XSetWMProtocols)(display, window, protocols.as_mut_ptr(), protocols.len() as c_int) };

    // let the window manager know this is a dialog box.
    let wm_window_type_str = CString::new("_NET_WM_WINDOW_TYPE").unwrap();
    let wm_window_type_dialog_str = CString::new("_NET_WM_WINDOW_TYPE_DIALOG").unwrap();
    let wm_window_type = unsafe { (xlib.XInternAtom)(display, wm_window_type_str.as_ptr(), xlib::False) };
    let wm_window_type_dialog = unsafe { (xlib.XInternAtom)(display, wm_window_type_dialog_str.as_ptr(), xlib::False) };
    let wm_window_type_dialog = &wm_window_type_dialog as *const u64 as *const u8;
    unsafe { (xlib.XChangeProperty)(display, window, wm_window_type, xlib::XA_ATOM, 32, xlib::PropModeReplace, wm_window_type_dialog, 1) };

    // specify events to use
    unsafe { (xlib.XSelectInput)(display, window, xlib::ExposureMask) };

    // create graphics context
    let gc = unsafe {
        let mut values: xlib::XGCValues = mem::uninitialized();
        (xlib.XCreateGC)(display, window, 0, &mut values)
    };

    // create font set
    let font_list = CString::new("-*-*-medium-r-normal--*-120-*-*-*-*-*-*").unwrap();
    let mut missing = ptr::null_mut();
    let mut num_missing = 0;
    let mut foo = ptr::null_mut();
    let font_set = unsafe { (xlib.XCreateFontSet)(display, font_list.as_ptr() as *const c_char, &mut missing, &mut num_missing, &mut foo) };

    // show window.
    unsafe { (xlib.XMapWindow)(display, window) };

    // split message into lines
    let lines: Vec<_> = message.lines().map(|x| CString::new(x)).filter_map(|x| x.ok()).collect();

    // determine line height
    let mut overall_ink = XRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut overall_logical = overall_ink.clone();
    let mut line_height = 1;
    for line in lines.iter() {
        unsafe { (xlib.Xutf8TextExtents)(font_set, message.as_ptr() as *const c_char, line.to_bytes().len() as i32, &mut overall_ink as *mut XRectangle, &mut overall_logical as *mut XRectangle) };
        if overall_logical.height as i32 > line_height {
            line_height = overall_logical.height as i32;
        }
    }
    line_height += 2;

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
            _ => ()
        }

        // redraw text
        for (i, line) in lines.iter().enumerate() {
            unsafe { (xlib.Xutf8DrawString)(display, window, font_set, gc, 10, (i as i32 + 1) * line_height, line.as_ptr() as *const c_char, line.to_bytes().len() as i32) };
        }
    }

    // shut down.
    unsafe { (xlib.XCloseDisplay)(display) };
}
