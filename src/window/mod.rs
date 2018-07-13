//! Contains the create_window function

#[cfg(all(target_os = "windows", feature = "gui"))]
#[path = "windows.rs"]
mod window_impl;

#[cfg(all(target_os = "linux", feature = "gui"))]
#[path = "linux.rs"]
mod window_impl;

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
mod window_impl {
  pub(crate) fn create_window(_: String) {}
}

use std::path::Path;
use Metadata;

/// Generate the human-panic mesage from the passed path and Metadata and
/// display it in a native OS window. GUI applications will need to make use
/// of this as they may not have a terminal to display in.
#[allow(unused)]
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn create_window<P: AsRef<Path>>(file_path: Option<P>, meta: &Metadata) {
  #[cfg(feature = "gui")]
  {
    use std::io::{Cursor, Read};
    use write_msg;

    let mut buffer = Cursor::new(vec![]);
    write_msg(file_path, meta, &mut buffer).expect(
      "human-panic: generating error message for GUI failed: write_msg",
    );
    buffer.set_position(0);

    let mut message = String::new();
    buffer.read_to_string(&mut message).expect(
      "human-panic: generating error message for GUI failed: read_to_string",
    );

    window_impl::create_window(message);
  }
}
