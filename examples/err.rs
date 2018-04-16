#[macro_use]
extern crate failure;

#[macro_use]
extern crate human_panic;

fn main() {
  setup_panic!();
  bail!("Ooops");
}
