#[test]
#[cfg_attr(debug_assertions, ignore)]
fn release() {
    snapbox::cmd::Command::new(snapbox::cmd::cargo_bin!("custom-panic-test"))
        .assert()
        .stderr_eq(snapbox::str![[r#"
Well, this is embarrassing.

custom-panic-test had a problem and crashed. To help us diagnose the problem you can send us a crash report.

We have generated a report file at "[..].toml". Submit an issue or email with the subject of "custom-panic-test Crash Report" and include the report as an attachment.

- Homepage: www.mycompany.com
- Authors: My Company Support <support@mycompany.com>

To submit the crash report:

- Open a support request by email to support@mycompany.com

We take privacy seriously, and do not perform any automated error collection. In order to improve the software, we rely on people to submit reports.

Thank you kindly!

"#]])
        .code(101);
}

#[test]
#[cfg_attr(not(debug_assertions), ignore)]
fn debug() {
    snapbox::cmd::Command::new(snapbox::cmd::cargo_bin!("custom-panic-test"))
        .assert()
        .stderr_eq(snapbox::str![[r#"
thread 'main' panicked at tests/custom-panic/src/main.rs:11:5:
OMG EVERYTHING IS ON FIRE!!!
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

"#]])
        .code(101);
}
