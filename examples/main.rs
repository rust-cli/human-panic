extern crate human_panic;

use human_panic::set_hook;

fn main() {
  set_hook();
  panic!("oops");
}
