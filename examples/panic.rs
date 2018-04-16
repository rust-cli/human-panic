#[macro_use]
extern crate human_panic;

fn main() {
  setup_panic!();

  println!("A normal log message");
  panic!("OMG EVERYTHING IS ON FIRE!!!");
}
