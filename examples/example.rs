#![windows_subsystem = "windows"] // disable the terminal in windows, like a real gui app

#[macro_use]
extern crate human_panic;

fn main() {
  setup_panic!(Metadata {
    name: "Human Panic Window Example".into(),
    version: env!("CARGO_PKG_VERSION").into(),
    authors: "ルカス".into(), // Can the GUI handle UTF8?
    homepage: "https://github.com/rust-clique/human-panic/issues".into(),
  });
  panic!("oh no");
}
