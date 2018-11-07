extern crate failure;
extern crate os_type;
extern crate serde;
extern crate tempdir;
extern crate toml;
extern crate uuid;

use self::failure::Error;
use self::uuid::Uuid;
use backtrace::Backtrace;
use std::borrow::Cow;
use std::fmt::Write as FmtWrite;
use std::mem;
use std::{env, fs::File, io::Write, path::Path, path::PathBuf};

/// Method of failure.
#[derive(Debug, Serialize, Clone, Copy)]
pub enum Method {
  Panic,
}

#[derive(Debug, Serialize)]
pub struct Report {
  name: String,
  operating_system: Cow<'static, str>,
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
    let operating_system = if cfg!(windows) {
      "windows".into()
    } else {
      let platform = os_type::current_platform();
      format!("unix:{:?}", platform.os_type).into()
    };

    //We skip non-user code frames, including Backtrace::new()
    const SKIP_FRAMES_NUM: usize = 8;
    //Code is based on backtrace source
    const HEX_WIDTH: usize = mem::size_of::<usize>() * 2 + 2;

    let mut backtrace = String::new();

    for (idx, frame) in Backtrace::new()
      .frames()
      .iter()
      .skip(SKIP_FRAMES_NUM)
      .enumerate()
    {
      let ip = frame.ip();
      let _ = write!(backtrace, "\n{:4}: {:2$?}", idx, ip, HEX_WIDTH);

      let symbols = frame.symbols();
      if symbols.len() == 0 {
        let _ = write!(backtrace, " - <unresolved>");
      }

      for (idx, symbol) in symbols.iter().enumerate() {
        if idx != 0 {
          let _ = write!(backtrace, "\n      {:1$}", "", HEX_WIDTH);
        }

        if let Some(name) = symbol.name() {
          let _ = write!(backtrace, " - {}", name);
        } else {
          let _ = write!(backtrace, " - <unknown>");
        }

        if let (Some(file), Some(line)) = (symbol.filename(), symbol.lineno()) {
          let _ = write!(
            backtrace,
            "\n      {:3$}at {}:{}",
            "",
            file.display(),
            line,
            HEX_WIDTH
          );
        }
      }
    }

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

  pub fn serialize(&self) -> Option<String> {
    toml::to_string_pretty(&self).ok()
  }

  /// Write a file to disk.
  pub fn persist(&self) -> Result<PathBuf, Error> {
    let uuid = Uuid::new_v4().to_hyphenated().to_string();
    let tmp_dir = env::temp_dir();
    let tmp_dir = match tmp_dir.to_str() {
      Some(dir) => dir,
      None => bail!("Could not create a tmp directory for a report."),
    };
    let file_name = format!("report-{}.toml", &uuid);
    let file_path = Path::new(tmp_dir).join(file_name);
    let mut file = File::create(&file_path)?;
    let toml = self.serialize().unwrap();
    file.write_all(toml.as_bytes())?;
    Ok(file_path)
  }
}
