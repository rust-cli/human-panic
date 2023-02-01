#[test]
#[cfg_attr(debug_assertions, ignore)]
fn release() {
    snapbox::cmd::Command::new(snapbox::cmd::cargo_bin!("single-panic-test"))
    .assert()
    .stderr_matches(
      "\
Well, this is embarrassing.

single-panic-test had a problem and crashed. To help us diagnose the problem you can send us a crash report.

We have generated a report file at \"[..].toml\". Submit an issue or email with the subject of \"single-panic-test Crash Report\" and include the report as an attachment.

- Authors: Human Panic Authors <human-panic-crate@example.com>

We take privacy seriously, and do not perform any automated error collection. In order to improve the software, we rely on people to submit reports.

Thank you kindly!
",
    )
    .code(101);
}

#[test]
#[cfg_attr(not(debug_assertions), ignore)]
fn debug() {
    snapbox::cmd::Command::new(snapbox::cmd::cargo_bin!("single-panic-test"))
        .assert()
        .stderr_matches(
            "\
thread 'main' panicked at 'OMG EVERYTHING IS ON FIRE!!!', tests/single-panic/src/main.rs:7:3
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
",
        )
        .code(101);
}
