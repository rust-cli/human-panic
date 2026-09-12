use std::io::Result as IoResult;
use std::panic::PanicHookInfo;
use std::path::{Path, PathBuf};

use crate::Metadata;
use crate::report::Report;

#[doc(hidden)]
pub fn setup_panic(meta: impl Fn() -> Metadata) {
    #![allow(deprecated)]

    #[allow(unused_imports)]
    use std::panic;

    match PanicStyle::default() {
        PanicStyle::Debug => {}
        PanicStyle::Human => {
            let meta = meta();

            panic::set_hook(Box::new(move |info: &PanicHookInfo<'_>| {
                let report = Report::with_panic(&meta, info);
                let file_path = if is_ci() { None } else { report.persist().ok() };
                if file_path.is_none() {
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
                }
                print_msg(file_path.as_deref(), &meta)
                    .expect("human-panic: printing error message to console failed");
            }));
        }
    }
}

/// Returns whether we are running in a CI environment.
fn is_ci() -> bool {
    std::env::var_os("CI").is_some()
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
            Self::Debug
        } else {
            match ::std::env::var("RUST_BACKTRACE") {
                Ok(_) => Self::Debug,
                Err(_) => Self::Human,
            }
        }
    }
}

/// Utility function that prints a message to our human users
pub fn print_msg<P: AsRef<Path>>(file_path: Option<P>, meta: &Metadata) -> IoResult<()> {
    #[cfg(feature = "color")]
    {
        use std::io::Write as _;

        let stderr = anstream::stderr();
        let mut stderr = stderr.lock();

        write!(stderr, "{}", anstyle::AnsiColor::Red.render_fg())?;
        write_msg(&mut stderr, file_path, meta)?;
        write!(stderr, "{}", anstyle::Reset.render())?;
    }

    #[cfg(not(feature = "color"))]
    {
        let stderr = std::io::stderr();
        let mut stderr = stderr.lock();

        write_msg(&mut stderr, file_path, meta)?;
    }

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
        repository,
        support,
        ..
    } = meta;

    writeln!(
        buffer,
        "{name} had a problem and crashed. To help us diagnose the \
     problem you can send us a crash report.\n"
    )?;
    if let Some(file_path) = file_path {
        writeln!(
            buffer,
            "We have generated a report file at \"{}\". Submit an \
     issue or email with the subject of \"{} Crash Report\" and include the \
     report as an attachment.\n",
            file_path.as_ref().display(),
            name
        )?;
    }

    if let Some(homepage) = homepage {
        writeln!(buffer, "- Homepage: {homepage}")?;
    } else if let Some(repository) = repository {
        writeln!(buffer, "- Repository: {repository}")?;
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
pub fn handle_dump(meta: &Metadata, panic_info: &PanicHookInfo<'_>) -> Option<PathBuf> {
    let report = Report::with_panic(meta, panic_info);

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
