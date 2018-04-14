#![feature(external_doc)]
#![doc(include = "../README.md")]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

extern crate unindent;

use std::panic;
use unindent::unindent;

/// Set a custom panic handler.
// Cargo env vars available:
// https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
pub fn set_hook() {
  panic::set_hook(Box::new(|_panic_info| {
    let _version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");
    let authors = env!("CARGO_PKG_AUTHORS");
    let homepage = env!("CARGO_PKG_HOMEPAGE");

    let msg = format!(r#"
      Well, this is embarrasing.

      {} had a problem and crashed. To help us diagnose the problem you can send us a crash report.

      We have generated a report file at "/tmp/example/panic-2018.log". Submit an issue or email with the subject of "{} Crash Report" and include the report as an attachment.
    "#, name, name);

    println!("{}", unindent(&msg));
    if !homepage.is_empty() {
      println!("- Homepage: {}", homepage);
    }
    if !authors.is_empty() {
      println!("- Authors: {}", authors);
    }
    println!("\nWe take privacy seriously, and do not perform any automated error collection. That means in order to improve the software, we rely on people to submit reports.");
    println!("\nThank you kindly!");
  }));
}
