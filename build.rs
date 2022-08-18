use std::process::{Command, Stdio};

fn main() {
    let mut version = String::new();
    // Cargo passes its binary path as the environment variable CARGO.
    let cargo_path = env!("CARGO");
    let command = Command::new(cargo_path)
        .arg("version")
        .stdout(Stdio::piped())
        .spawn();
    if let Ok(command) = command {
        let output = command.wait_with_output();
        if let Ok(output) = output {
            version = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();
        }
    }
    println!("cargo:rustc-env=CARGO_VERSION={}", version);
}
