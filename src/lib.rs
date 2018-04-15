#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate termcolor;

mod report;

use failure::Error;
use report::{Method, Report};
use std::panic;

/// Catch any error handlers that occur, and
// Cargo env vars available:
// https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
pub fn catch_unwind<F: FnOnce() -> Result<(), Error>>(f: F) {
  panic::set_hook(Box::new(|_panic_info| {
    let report = Report::new(Method::Panic);
    let file_path = report.persist().unwrap();
    print_msg(file_path).unwrap();
  }));

  if let Err(_) = f() {
    let report = Report::new(Method::Err);
    let file_path = report.persist().unwrap();
    print_msg(file_path).unwrap();
  }
}

use std::io::{Result as IoResult, Write};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

fn print_msg(file_path: String) -> IoResult<()> {
  let stderr = BufferWriter::stderr(ColorChoice::Auto);
  let mut buffer = stderr.buffer();
  buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;

  let _version = env!("CARGO_PKG_VERSION");
  let name = env!("CARGO_PKG_NAME");
  let authors = env!("CARGO_PKG_AUTHORS");
  let homepage = env!("CARGO_PKG_HOMEPAGE");

  writeln!(&mut buffer, "Well, this is embarrasing.\n")?;
  writeln!(&mut buffer, "{} had a problem and crashed. To help us diagnose the problem you can send us a crash report.\n", name)?;
  writeln!(&mut buffer, "We have generated a report file at \"{}\". Submit an issue or email with the subject of \"{} Crash Report\" and include the report as an attachment.\n", &file_path, name)?;

  if !homepage.is_empty() {
    writeln!(&mut buffer, "- Homepage: {}", homepage)?;
  }
  if !authors.is_empty() {
    writeln!(&mut buffer, "- Authors: {}", authors)?;
  }
  writeln!(&mut buffer, "\nWe take privacy seriously, and do not perform any automated error collection. In order to improve the software, we rely on people to submit reports.\n")?;
  writeln!(&mut buffer, "Thank you kindly!")?;

  stderr.print(&buffer).unwrap();

  Ok(())
}
