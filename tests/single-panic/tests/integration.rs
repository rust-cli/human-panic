#[test]
#[cfg_attr(debug_assertions, ignore)]
fn release() {
    let root = snapbox::dir::DirRoot::mutable_temp().unwrap();
    let root_path = root.path().unwrap();

    #[cfg(unix)]
    let envs = [("TMPDIR", root_path)];
    #[cfg(not(unix))]
    let envs: [(&str, &str); 0] = [];

    snapbox::cmd::Command::new(snapbox::cmd::cargo_bin!("single-panic-test"))
        .envs(envs)
        .assert()
        .stderr_eq(snapbox::str![[r#"
Well, this is embarrassing.

single-panic-test had a problem and crashed. To help us diagnose the problem you can send us a crash report.

We have generated a report file at "[..].toml". Submit an issue or email with the subject of "single-panic-test Crash Report" and include the report as an attachment.

- Authors: Human Panic Authors <human-panic-crate@example.com>

We take privacy seriously, and do not perform any automated error collection. In order to improve the software, we rely on people to submit reports.

Thank you kindly!

"#]])
        .code(101);

    #[cfg(unix)]
    {
        let mut files = root_path
            .read_dir()
            .unwrap()
            .map(|e| {
                let e = e.unwrap();
                let path = e.path();
                let content = std::fs::read_to_string(&path);
                (path, content)
            })
            .collect::<Vec<_>>();
        assert_eq!(files.len(), 1, "{files:?}");
        let (_, report) = files.pop().unwrap();
        let report = report.unwrap();
        snapbox::assert_data_eq!(
            report,
            snapbox::str![[r#"
"name" = "single-panic-test"
"operating_system" = "[..]"
"crate_version" = "0.1.0"
"explanation" = """
Panic occurred in file 'tests/single-panic/src/main.rs' at line [..]
"""
"cause" = "OMG EVERYTHING IS ON FIRE!!!"
"method" = "Panic"
"backtrace" = """
...
"""

"#]]
        );
    }

    root.close().unwrap();
}

#[test]
#[cfg_attr(not(debug_assertions), ignore)]
fn debug() {
    let root = snapbox::dir::DirRoot::mutable_temp().unwrap();
    let root_path = root.path().unwrap();

    #[cfg(unix)]
    let envs = [("TMPDIR", root_path)];
    #[cfg(not(unix))]
    let envs: [(&str, &str); 0] = [];

    snapbox::cmd::Command::new(snapbox::cmd::cargo_bin!("single-panic-test"))
        .envs(envs)
        .assert()
        .stderr_eq(snapbox::str![[r#"
thread 'main' panicked at tests/single-panic/src/main.rs:[..]:
OMG EVERYTHING IS ON FIRE!!!
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

"#]])
        .code(101);

    #[cfg(unix)]
    {
        let files = root_path
            .read_dir()
            .unwrap()
            .map(|e| {
                let e = e.unwrap();
                let path = e.path();
                let content = std::fs::read_to_string(&path);
                (path, content)
            })
            .collect::<Vec<_>>();
        assert!(files.is_empty(), "{files:?}");
    }

    root.close().unwrap();
}
