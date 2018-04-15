#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]

use std::panic;
use std::error::Error;

/// Catch any error handlers that occur, and
// Cargo env vars available:
// https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
pub fn catch_unwind<F: FnOnce() -> Result<(), Box<Error>>>(f: F) {
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

  eprintln!("Well, this is embarrasing.\n");
  eprintln!("{} had a problem and crashed. To help us diagnose the problem you can send us a crash report.\n", name);
  eprintln!("We have generated a report file at \"<reports not generated yet>\". Submit an issue or email with the subject of \"{} Crash Report\" and include the report as an attachment.\n", name);

  if !homepage.is_empty() {
    eprintln!("- Homepage: {}", homepage);
  }
  if !authors.is_empty() {
    eprintln!("- Authors: {}", authors);
  }
  eprintln!("\nWe take privacy seriously, and do not perform any automated error collection. In order to improve the software, we rely on people to submit reports.\n");
  eprintln!("Thank you kindly!");
}
