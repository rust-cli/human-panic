use human_panic::setup_panic;

#[derive(Debug, PartialEq)]
struct Metadata {
    test: bool,
}

mod panic {
    pub fn what() {}
}

fn main() {
    panic::what();
    let prev = Metadata { test: true };

    setup_panic!();

    let next = Metadata { test: false };

    assert_ne!(prev, next);
    panic::what();

    println!("A normal log message");
    panic!("OMG EVERYTHING IS ON FIRE!!!");
}
