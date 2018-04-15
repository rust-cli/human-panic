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
  panic::set_hook(Box::new(|panic_info| {
    let mut expl = String::new();

    let payload = panic_info.payload().downcast_ref::<&str>();
    if let Some(payload) = payload {
      expl.push_str(&format!("Cause: {}.", &payload));
    }

    match panic_info.location() {
      Some(location) => expl.push_str(&format!(
        "Panic occurred in file '{}' at line {}\n",
        location.file(),
        location.line()
      )),
      None => expl.push_str("Panic location uknown.\n"),
    }

    let report = Report::new(Method::Panic, expl);
    let file_path = report
      .persist()
      .expect("human-panic: writing report failed");
    print_msg(file_path)
      .expect("human-panic: printing error message to console failed");
  }));

  if let Err(err) = f() {
    let mut expl = String::new();
    expl.push_str(&format!(
      "Cause: {}\n\n",
      format!("{}", err.cause())
    ));
    expl.push_str(&format!(
      "Backtrace: {}\n\n",
      format!("{}", err.backtrace())
    ));
    let report = Report::new(Method::Err, expl);
    let file_path = report
      .persist()
      .expect("human-panic: writing report failed");
    print_msg(file_path)
      .expect("human-panic: printing error message to console failed");
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
