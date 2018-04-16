#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate termcolor;

mod report;
use report::{Method, Report};

use failure::Error as FailError;
use std::io::{Result as IoResult, Write};
use std::panic::PanicInfo;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

/// A convenient metadata struct that describes a crate
pub struct Metadata<'a> {
  version: &'a str,
  name: &'a str,
  authors: &'a str,
  homepage: &'a str,
}

/// Setup the human panic hook that will make all panics
/// as beautiful as your shitty code.
#[macro_export]
macro_rules! setup_panic {
  () => {
    use human_panic::*;
    use std::env;
    use std::panic::{self, PanicInfo};

    let meta = Metadata {
    version = env!("CARGO_PKG_VERSION"),
    name = env!("CARGO_PKG_NAME"),
    authors = env!("CARGO_PKG_AUTHORS"),
    homepage = env!("CARGO_PKG_HOMEPAGE"),
    };


    panic::set_hook(Box::new(move |info: &PanicInfo| {
      let file_path =
        handle_dump(info).expect("human-panic: dumping logs to disk failed");

      print_msg(file_path, meta)
        .expect("human-panic: printing error message to console failed");
    }));
  };
}

/// Utility function that prints a message to our human users
pub fn print_msg(file_path: String, meta: &Metadata) -> IoResult<()> {
  let (_version, name, authors, homepage) = (
    meta.version,
    meta.name,
    meta.authors,
    meta.homepage,
  );

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
pub fn handle_dump(panic_info: &PanicInfo) -> Result<String, FailError> {
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
  return report.persist();
}
