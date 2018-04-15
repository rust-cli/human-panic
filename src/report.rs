extern crate failure;
extern crate os_type;
extern crate serde;
extern crate tempdir;
extern crate toml;
extern crate uuid;

use self::failure::Error;
use self::uuid::Uuid;
use std::{env, fs::File, io::Write};

/// Method of failure.
#[derive(Debug, Serialize)]
pub enum Method {
  Panic,
  Err,
}

#[derive(Debug, Serialize)]
pub struct Report {
  name: String,
  operating_system: String,
  crate_version: String,
  explanation: String,
  method: Method,
}

impl Report {
  /// Create a new instance.
  pub fn new(method: Method, explanation: String) -> Self {
    let operating_system;
    if cfg!(windows) {
      operating_system = "windows".to_string();
    } else {
      let platform = os_type::current_platform();
      operating_system = format!("unix:{:?}", platform.os_type);
    }

    Self {
      crate_version: env!("CARGO_PKG_VERSION").to_string(),
      name: env!("CARGO_PKG_NAME").to_string(),
      operating_system,
      method,
      explanation,
    }
  }

  /// Write a file to disk.
  pub fn persist(&self) -> Result<String, Error> {
    let uuid = Uuid::new_v4().hyphenated().to_string();
    let tmp_dir = env::temp_dir();
    let tmp_dir = match tmp_dir.to_str() {
      Some(dir) => dir,
      None => bail!("Could not create a tmp directory for a report."),
    };
    let file_path = format!("{}/report-{}.toml", tmp_dir, &uuid);
    let mut file = File::create(&file_path)?;
    let toml = toml::to_string(&self)?;
    file.write_all(toml.as_bytes())?;
    Ok(file_path)
  }
}
