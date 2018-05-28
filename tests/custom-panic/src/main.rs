#[macro_use]
extern crate human_panic;

use human_panic::Metadata;

fn main() {
  setup_panic!(Metadata {
    name: 
    authors: "My Company Support <support@mycompany.com",
    homepage: "support.mycompany.com"
    
  });

  println!("A normal log message");
  panic!("OMG EVERYTHING IS ON FIRE!!!");
}
