//! This module encapsulates the report of a failure event.
//!
//! A `Report` contains the metadata collected about the event
//! to construct a helpful error message.

use backtrace::Backtrace;
use serde_derive::Serialize;
use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::mem;
use std::{env, path::Path, path::PathBuf};
use uuid::Uuid;

/// Method of failure.
#[derive(Debug, Serialize, Clone, Copy)]
#[non_exhaustive]
pub enum Method {
    /// Failure caused by a panic.
    Panic,
}

/// Contains metadata about the crash like the backtrace and
/// information about the crate and operating system. Can
/// be used to be serialized and persisted or printed as
/// information to the user.
#[derive(Debug, Serialize)]
pub struct Report {
    name: String,
    operating_system: String,
    crate_version: String,
    explanation: String,
    cause: String,
    method: Method,
    backtrace: String,
}

impl Report {
    /// Create a new instance.
    pub fn new(
        name: &str,
        version: &str,
        method: Method,
        explanation: String,
        cause: String,
    ) -> Self {
        let cpu_arch = sysinfo::System::cpu_arch();
        let operating_system =
            sysinfo::System::long_os_version().unwrap_or_else(|| "unknown".to_owned());
        let operating_system = format!("{operating_system} [{cpu_arch}]");
        let backtrace = render_backtrace();

        Self {
            crate_version: version.into(),
            name: name.into(),
            operating_system,
            method,
            explanation,
            cause,
            backtrace,
        }
    }

    /// Serialize the `Report` to a TOML string.
    pub fn serialize(&self) -> Option<String> {
        toml::to_string_pretty(&self).ok()
    }

    /// Write a file to disk.
    pub fn persist(&self) -> Result<PathBuf, Box<dyn Error + 'static>> {
        let uuid = Uuid::new_v4().hyphenated().to_string();
        let tmp_dir = env::temp_dir();
        let file_name = format!("report-{}.toml", &uuid);
        let file_path = Path::new(&tmp_dir).join(file_name);
        let toml = self.serialize().expect("only using toml-compatible types");
        std::fs::write(&file_path, toml.as_bytes())?;
        Ok(file_path)
    }
}

fn render_backtrace() -> String {
    //We take padding for address and extra two letters
    //to pad after index.
    #[allow(unused_qualifications)] // needed for pre-1.80 MSRV
    const HEX_WIDTH: usize = mem::size_of::<usize>() * 2 + 2;
    //Padding for next lines after frame's address
    const NEXT_SYMBOL_PADDING: usize = HEX_WIDTH + 6;

    let mut backtrace = String::new();

    //Here we iterate over backtrace frames
    //(each corresponds to function's stack)
    //We need to print its address
    //and symbol(e.g. function name),
    //if it is available
    let bt = Backtrace::new();
    let symbols = bt
        .frames()
        .iter()
        .flat_map(|frame| {
            let symbols = frame.symbols();
            if symbols.is_empty() {
                vec![(frame, None, "<unresolved>".to_owned())]
            } else {
                symbols
                    .iter()
                    .map(|s| {
                        (
                            frame,
                            Some(s),
                            s.name()
                                .map(|n| n.to_string())
                                .unwrap_or_else(|| "<unknown>".to_owned()),
                        )
                    })
                    .collect::<Vec<_>>()
            }
        })
        .collect::<Vec<_>>();
    let begin_unwind = "rust_begin_unwind";
    let begin_unwind_start = symbols
        .iter()
        .position(|(_, _, n)| n == begin_unwind)
        .unwrap_or(0);
    for (entry_idx, (frame, symbol, name)) in symbols.iter().skip(begin_unwind_start).enumerate() {
        let ip = frame.ip();
        let _ = writeln!(backtrace, "{entry_idx:4}: {ip:HEX_WIDTH$?} - {name}");
        if let Some(symbol) = symbol {
            //See if there is debug information with file name and line
            if let (Some(file), Some(line)) = (symbol.filename(), symbol.lineno()) {
                let _ = writeln!(
                    backtrace,
                    "{:3$}at {}:{}",
                    "",
                    file.display(),
                    line,
                    NEXT_SYMBOL_PADDING
                );
            }
        }
    }

    backtrace
}
