//! Panic messages for humans
//!
//! Handles panics by calling
//! [`std::panic::set_hook`](https://doc.rust-lang.org/std/panic/fn.set_hook.html)
//! to make errors nice for humans.
//!
//! ## Why?
//! When you're building a CLI, polish is super important. Even though Rust is
//! pretty great at safety, it's not unheard of to access the wrong index in a
//! vector or have an assert fail somewhere.
//!
//! When an error eventually occurs, you probably will want to know about it. So
//! instead of just providing an error message on the command line, we can create a
//! call to action for people to submit a report.
//!
//! This should empower people to engage in communication, lowering the chances
//! people might get frustrated. And making it easier to figure out what might be
//! causing bugs.
//!
//! ### Default Output
//!
//! ```txt
//! thread 'main' panicked at 'oops', examples/main.rs:2:3
//! note: Run with `RUST_BACKTRACE=1` for a backtrace.
//! ```
//!
//! ### Human-Panic Output
//!
//! ```txt
//! Well, this is embarrassing.
//!
//! human-panic had a problem and crashed. To help us diagnose the problem you can send us a crash report.
//!
//! We have generated a report file at "/var/folders/zw/bpfvmq390lv2c6gn_6byyv0w0000gn/T/report-8351cad6-d2b5-4fe8-accd-1fcbf4538792.toml". Submit an issue or email with the subject of "human-panic Crash Report" and include the report as an attachment.
//!
//! - Homepage: https://github.com/rust-ci/human-panic
//! - Authors: Yoshua Wuyts <yoshuawuyts@gmail.com>
//!
//! We take privacy seriously, and do not perform any automated error collection. In order to improve the software, we rely on people to submit reports.
//!
//! Thank you kindly!

#![cfg_attr(feature = "nightly", feature(panic_info_message))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

pub mod report;
use report::{Method, Report};

use std::borrow::Cow;
use std::io::Result as IoResult;
#[allow(deprecated)]
use std::panic::PanicInfo;
use std::path::{Path, PathBuf};

/// A convenient metadata struct that describes a crate
///
/// See [`metadata!`]
pub struct Metadata {
    pub name: Cow<'static, str>,
    pub version: Cow<'static, str>,
    pub authors: Option<Cow<'static, str>>,
    pub homepage: Option<Cow<'static, str>>,
    pub support: Option<Cow<'static, str>>,
}

impl Metadata {
    /// See [`metadata!`]
    pub fn new(name: impl Into<Cow<'static, str>>, version: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            authors: None,
            homepage: None,
            support: None,
        }
    }

    /// The list of authors of the crate
    pub fn authors(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        let value = value.into();
        if !value.is_empty() {
            self.authors = value.into();
        }
        self
    }

    /// The URL of the crate's website
    pub fn homepage(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        let value = value.into();
        if !value.is_empty() {
            self.homepage = value.into();
        }
        self
    }

    /// The support information
    pub fn support(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        let value = value.into();
        if !value.is_empty() {
            self.support = value.into();
        }
        self
    }
}

/// Initialize [`Metadata`]
#[macro_export]
macro_rules! metadata {
    () => {{
        $crate::Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
            .authors(env!("CARGO_PKG_AUTHORS").replace(":", ", "))
            .homepage(env!("CARGO_PKG_HOMEPAGE"))
    }};
}

/// `human-panic` initialisation macro
///
/// You can either call this macro with no arguments `setup_panic!()` or
/// with a Metadata struct, if you don't want the error message to display
/// the values used in your `Cargo.toml` file.
///
/// The Metadata struct can't implement `Default` because of orphan rules, which
/// means you need to provide all fields for initialisation.
///
/// The macro should be called from within a function, for example as the first line of the
/// `main()` function of the program.
///
/// ```
/// use human_panic::setup_panic;
/// use human_panic::Metadata;
///
/// setup_panic!(Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
///     .authors("My Company Support <support@mycompany.com>")
///     .homepage("support.mycompany.com")
///     .support("- Open a support request by email to support@mycompany.com")
/// );
/// ```
#[macro_export]
macro_rules! setup_panic {
    ($meta:expr) => {{
        $crate::setup_panic(|| $meta, None);
    }};

    ($meta:expr, $print_fn:expr) => {{
        $crate::setup_panic(|| $meta, Some($print_fn));
    }};

    () => {
        $crate::setup_panic(|| $crate::metadata!(), None);
    };
}

pub type WriteFunc = fn (&mut dyn std::io::Write, Option<&Path>, &Metadata) -> IoResult<()>;

#[doc(hidden)]
pub fn setup_panic(meta: impl Fn() -> Metadata, write_func: Option<WriteFunc>) {
    #![allow(deprecated)]

    #[allow(unused_imports)]
    use std::panic;

    match PanicStyle::default() {
        PanicStyle::Debug => {}
        PanicStyle::Human => {
            let meta = meta();

            panic::set_hook(Box::new(move |info: &PanicInfo<'_>| {
                let file_path = handle_dump(&meta, info);
                let write_func = write_func.unwrap_or(|mut buf,fp,meta| write_msg(&mut buf,fp,meta));
                print_msg(file_path, &meta, write_func)
                    .expect("human-panic: printing error message to console failed");
            }));
        }
    }
}

/// Style of panic to be used
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PanicStyle {
    /// Normal panic
    Debug,
    /// Human-formatted panic
    Human,
}

impl Default for PanicStyle {
    fn default() -> Self {
        if cfg!(debug_assertions) {
            PanicStyle::Debug
        } else {
            match ::std::env::var("RUST_BACKTRACE") {
                Ok(_) => PanicStyle::Debug,
                Err(_) => PanicStyle::Human,
            }
        }
    }
}

/// Utility function that prints a message to our human users
#[cfg(feature = "color")]
pub fn print_msg<P: AsRef<Path>>(file_path: Option<P>, meta: &Metadata, write_func: WriteFunc) -> IoResult<()> {
    use std::io::Write as _;

    let stderr = anstream::stderr();
    let mut stderr = stderr.lock();

    write!(stderr, "{}", anstyle::AnsiColor::Red.render_fg())?;
    let fp = file_path.as_ref().map(|fp| fp.as_ref());
    write_func(&mut stderr, fp, meta)?;
    write!(stderr, "{}", anstyle::Reset.render())?;

    Ok(())
}

#[cfg(not(feature = "color"))]
pub fn print_msg<P: AsRef<Path>>(file_path: Option<P>, meta: &Metadata, write_func: WriteFunc) -> IoResult<()> {
    let stderr = std::io::stderr();
    let mut stderr = stderr.lock();

    let fp = file_path.as_ref().map(|fp| fp.as_ref());
    write_func(&mut stderr, fp, meta)?;

    Ok(())
}

fn write_msg<P: AsRef<Path>>(
    buffer: &mut impl std::io::Write,
    file_path: Option<P>,
    meta: &Metadata,
) -> IoResult<()> {
    let Metadata {
        name,
        authors,
        homepage,
        support,
        ..
    } = meta;

    writeln!(buffer, "Well, this is embarrassing.\n")?;
    writeln!(
        buffer,
        "{name} had a problem and crashed. To help us diagnose the \
     problem you can send us a crash report.\n"
    )?;
    writeln!(
        buffer,
        "We have generated a report file at \"{}\". Submit an \
     issue or email with the subject of \"{} Crash Report\" and include the \
     report as an attachment.\n",
        match file_path {
            Some(fp) => format!("{}", fp.as_ref().display()),
            None => "<Failed to store file to disk>".to_owned(),
        },
        name
    )?;

    if let Some(homepage) = homepage {
        writeln!(buffer, "- Homepage: {homepage}")?;
    }
    if let Some(authors) = authors {
        writeln!(buffer, "- Authors: {authors}")?;
    }
    if let Some(support) = support {
        writeln!(buffer, "\nTo submit the crash report:\n\n{support}")?;
    }
    writeln!(
        buffer,
        "\nWe take privacy seriously, and do not perform any \
     automated error collection. In order to improve the software, we rely on \
     people to submit reports.\n"
    )?;
    writeln!(buffer, "Thank you kindly!")?;

    Ok(())
}

/// Utility function which will handle dumping information to disk
#[allow(deprecated)]
pub fn handle_dump(meta: &Metadata, panic_info: &PanicInfo<'_>) -> Option<PathBuf> {
    let mut expl = String::new();

    #[cfg(feature = "nightly")]
    let message = panic_info.message().map(|m| format!("{}", m));

    #[cfg(not(feature = "nightly"))]
    let message = match (
        panic_info.payload().downcast_ref::<&str>(),
        panic_info.payload().downcast_ref::<String>(),
    ) {
        (Some(s), _) => Some((*s).to_owned()),
        (_, Some(s)) => Some(s.to_owned()),
        (None, None) => None,
    };

    let cause = match message {
        Some(m) => m,
        None => "Unknown".into(),
    };

    match panic_info.location() {
        Some(location) => expl.push_str(&format!(
            "Panic occurred in file '{}' at line {}\n",
            location.file(),
            location.line()
        )),
        None => expl.push_str("Panic location unknown.\n"),
    }

    let report = Report::new(&meta.name, &meta.version, Method::Panic, expl, cause);

    if let Ok(f) = report.persist() {
        Some(f)
    } else {
        use std::io::Write as _;
        let stderr = std::io::stderr();
        let mut stderr = stderr.lock();

        let _ = writeln!(
            stderr,
            "{}",
            report
                .serialize()
                .expect("only doing toml compatible types")
        );
        None
    }
}
