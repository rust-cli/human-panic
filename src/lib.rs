#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]

extern crate backtrace;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate termcolor;

mod report;
use report::{Method, Report};

use std::borrow::Cow;
use std::io::{Result as IoResult, Write};
use std::panic::PanicInfo;
use std::path::{Path, PathBuf};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

/// A convenient metadata struct that describes a crate
pub struct Metadata {
  /// The crate version
  pub version: Cow<'static, str>,
  /// The crate name
  pub name: Cow<'static, str>,
  /// The list of authors of the crate
  pub authors: Cow<'static, str>,
  /// The URL of the crate's website
  pub homepage: Cow<'static, str>,
}

/// Setup the human panic hook that will make all panics
/// as beautiful as your shitty code.
#[macro_export]
macro_rules! setup_panic {
  () => {
    use human_panic::*;
    use std::panic::{self, PanicInfo};

    let meta = Metadata {
      version: env!("CARGO_PKG_VERSION").into(),
      name: env!("CARGO_PKG_NAME").into(),
      authors: env!("CARGO_PKG_AUTHORS").replace(":", ", ").into(),
      homepage: env!("CARGO_PKG_HOMEPAGE").into(),
    };

    panic::set_hook(Box::new(move |info: &PanicInfo| {
      let file_path = handle_dump(&meta, info);

      print_msg(file_path, &meta)
        .expect("human-panic: printing error message to console failed");
    }));
  };
}

/// Utility function that prints a message to our human users
pub fn print_msg<P: AsRef<Path>>(
  file_path: Option<P>,
  meta: &Metadata,
) -> IoResult<()> {
  let (_version, name, authors, homepage) =
    (&meta.version, &meta.name, &meta.authors, &meta.homepage);

  let stderr = BufferWriter::stderr(ColorChoice::Auto);
  let mut buffer = stderr.buffer();
  buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;

  writeln!(&mut buffer, "Well, this is embarrassing.\n")?;
  writeln!(
    &mut buffer,
    "{} had a problem and crashed. To help us diagnose the \
     problem you can send us a crash report.\n",
    name
  )?;
  writeln!(
    &mut buffer,
    "We have generated a report file at \"{}\". Submit an \
     issue or email with the subject of \"{} Crash Report\" and include the \
     report as an attachment.\n",
    match file_path {
      Some(fp) => format!("{}", fp.as_ref().display()),
      None => "<Failed to store file to disk>".to_string(),
    },
    name
  )?;

  if !homepage.is_empty() {
    writeln!(&mut buffer, "- Homepage: {}", homepage)?;
  }
  if !authors.is_empty() {
    writeln!(&mut buffer, "- Authors: {}", authors)?;
  }
  writeln!(
    &mut buffer,
    "\nWe take privacy seriously, and do not perform any \
     automated error collection. In order to improve the software, we rely on \
     people to submit reports.\n"
  )?;
  writeln!(&mut buffer, "Thank you kindly!")?;

  stderr.print(&buffer).unwrap();
  Ok(())
}

/// Utility function which will handle dumping information to disk
pub fn handle_dump(meta: &Metadata, panic_info: &PanicInfo) -> Option<PathBuf> {
  let mut expl = String::new();

  let payload = panic_info.payload().downcast_ref::<&str>();
  if let Some(payload) = payload {
    expl.push_str(&format!("Cause: {}. ", &payload));
  }

  match panic_info.location() {
    Some(location) => expl.push_str(&format!(
      "Panic occurred in file '{}' at line {}\n",
      location.file(),
      location.line()
    )),
    None => expl.push_str("Panic location unknown.\n"),
  }

  let report = Report::new(&meta.name, &meta.version, Method::Panic, expl);

  match report.persist() {
    Ok(f) => Some(f),
    Err(_) => {
      eprintln!("{}", report.serialize().unwrap());
      None
    }
  }
}
