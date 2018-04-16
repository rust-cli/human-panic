#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate termcolor;

mod report;
use std::panic::PanicInfo;
use std::io::{Result as IoResult, Write};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};


/// Setup the human panic hook that will make all panics
/// as beautiful as your shitty code.
#[macro_export]
macro_rules! setup_panic {
  () => {
    use std::env;
    use std::panic::{self, PanicInfo};
    use human_panic::*;
    let _version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");
    let authors = env!("CARGO_PKG_AUTHORS");
    let homepage = env!("CARGO_PKG_HOMEPAGE");

    panic::set_hook(Box::new(move |info: &PanicInfo| {
      let file_path = handle_dump(info);
      print_msg(file_path, _version, name, authors, homepage).unwrap();
    }));
  };
}

/// Utility function that prints a message to our human users
pub fn print_msg(file_path: String, _version: &str, name: &str, authors: &str, homepage: &str) -> IoResult<()> {
  let stderr = BufferWriter::stderr(ColorChoice::Auto);
  let mut buffer = stderr.buffer();
  buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;

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

/// Utility function which will handle dumping information to disk
/// TODO: Implement
pub fn handle_dump(_panic_info: &PanicInfo) -> String {
  let r = report::Report::new(report::Method::Err);
  return r.persist().unwrap();
}
