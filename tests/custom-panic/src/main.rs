#[macro_use]
extern crate human_panic;

fn main() {
  setup_panic!(Metadata {
    name: env!("CARGO_PKG_NAME").into(),
    version: env!("CARGO_PKG_VERSION").into(),
    authors: "My Company Support <support@mycompany.com".into(),
    homepage: "support.mycompany.com".into(),
    repository: "https://git.mycompany.com/repo.git".into(),
  });

  println!("A normal log message");
  panic!("OMG EVERYTHING IS ON FIRE!!!");
}
