use std::process::{Command, Stdio};

fn main() {
    let mut version = String::new();
    // Cargo passes rustc's binary path as the environment variable RUSTC
    let rustc_path = std::env::var("RUSTC").unwrap();
    let command = Command::new(rustc_path)
        .arg("-vV")
        .stdout(Stdio::piped())
        .spawn();
    if let Ok(command) = command {
        let output = command.wait_with_output();
        if let Ok(output) = output {
            version = String::from_utf8_lossy(&output.stdout)
                .trim()
                .replace("\n", "\\n")
                .to_string();
        }
    }
    println!("cargo:rustc-env=RUSTC_VERSION={}", version);
}
