#![windows_subsystem = "windows"]

#[macro_use]
extern crate human_panic;

fn main() {
  setup_panic!(Metadata {
    name: "Human Panic Window Example".into(),
    version: env!("CARGO_PKG_VERSION").into(),
    authors: "ルカス".into(),
    homepage: "https://github.com/rust-clique/human-panic/issues".into(),
    create_window: true,
  });
  panic!("oh no");
}
