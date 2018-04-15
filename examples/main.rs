extern crate human_panic;

use human_panic::catch_unwind;

fn main() {
  catch_unwind(|| {
    panic!("oops");
  });
}
