#[test]
#[cfg_attr(debug_assertions, ignore)]
fn release() {
    snapbox::cmd::Command::new(snapbox::cmd::cargo_bin!("custom-message-test"))
        .assert()
        .stderr_eq(snapbox::str![[r#"
A fatal error ocurred!
custom-message-test is DEAD :(
You can see details at [..].

Share your condolences: www.mycompany.com

"#]])
        .code(101);
}

#[test]
#[cfg_attr(not(debug_assertions), ignore)]
fn debug() {
    snapbox::cmd::Command::new(snapbox::cmd::cargo_bin!("custom-message-test"))
        .assert()
        .stderr_eq(snapbox::str![[r#"
thread 'main' panicked at tests/custom-message/src/main.rs:45:5:
OMG EVERYTHING IS ON FIRE!!!
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

"#]])
        .code(101);
}
