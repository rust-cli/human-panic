extern crate human_panic;
#[macro_use]
extern crate failure;

use human_panic::catch_unwind;

fn main() {
  catch_unwind(|| {
    bail!("Ooops");
  });
}
