use human_panic::metadata;
use human_panic::setup_panic;

fn main() {
    setup_panic!(metadata!()
        .authors("My Company Support <support@mycompany.com")
        .homepage("www.mycompany.com")
        .support("- Open a support request by email to support@mycompany.com"));

    println!("A normal log message");
    panic!("OMG EVERYTHING IS ON FIRE!!!");
}
