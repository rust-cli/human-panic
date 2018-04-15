#![feature(external_doc)]
#![doc(include = "../README.md")]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

extern crate console;
extern crate failure;
extern crate unindent;

use console::style;
use failure::Error;
use std::panic;
use unindent::unindent;

/// Catch any error handlers that occur, and
// Cargo env vars available:
// https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
pub fn catch_unwind<F: FnOnce() -> Result<(), Error>>(f: F) {
  panic::set_hook(Box::new(|_panic_info| {
    // TODO: create log report.
    print_msg();
  }));

  match f() {
    Ok(_) => {}
    _ => { /* TODO: create log report. */ }
  }
}

fn print_msg() {
  let _version = env!("CARGO_PKG_VERSION");
  let name = env!("CARGO_PKG_NAME");
  let authors = env!("CARGO_PKG_AUTHORS");
  let homepage = env!("CARGO_PKG_HOMEPAGE");

  let mut msg = unindent(&format!(r#"
      Well, this is embarrasing.

      {} had a problem and crashed. To help us diagnose the problem you can send us a crash report.

      We have generated a report file at "<reports not generated yet>". Submit an issue or email with the subject of "{} Crash Report" and include the report as an attachment.
    "#, name, name));
  msg.push_str("\n");

  if !homepage.is_empty() {
    msg.push_str(&format!("- Homepage: {}\n", homepage));
  }
  if !authors.is_empty() {
    msg.push_str(&format!("- Authors: {}\n", authors));
  }
  msg.push_str("\nWe take privacy seriously, and do not perform any automated error collection. In order to improve the software, we rely on people to submit reports.\n");
  msg.push_str("\nThank you kindly!");
  eprintln!("{}", style(msg).red());
}
