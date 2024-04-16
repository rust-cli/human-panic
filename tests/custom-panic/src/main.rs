use human_panic::{metadata, setup_panic};

fn main() {
    let mut metadata = metadata!();
    metadata.authors = Some("My Company Support <support@mycompany.com".into());
    metadata.homepage = Some("www.mycompany.com".into());
    metadata.support = Some("- Open a support request by email to support@mycompany.com".into());
    setup_panic!(metadata);

    println!("A normal log message");
    panic!("OMG EVERYTHING IS ON FIRE!!!");
}
