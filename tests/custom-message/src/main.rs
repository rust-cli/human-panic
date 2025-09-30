use std::path::Path;

use human_panic::metadata;
use human_panic::Metadata;
use human_panic::setup_panic;

fn custom_message(
    buf: &mut dyn std::io::Write,
    path: Option<&Path>,
    meta: &Metadata
) -> std::io::Result<()> {
    let Metadata {
        name,
        homepage,
        ..
    } = meta;

    writeln!(buf, "A fatal error ocurred!")?;
    writeln!(buf,"{name} is DEAD :(")?;
    writeln!(
        buf,
        "You can see details at \"{}\".",
        match path {
            Some(fp) => format!("{}", fp.display()),
            None => "<Failed to store file to disk>".to_owned(),
        },
    )?;

    if let Some(homepage) = homepage {
        writeln!(buf, "\nShare your condolences: {homepage}")?;
    }

    Ok(())
}

fn main() {
    setup_panic!(
        metadata!()
                .authors("My Company Support <support@mycompany.com>")
                .homepage("www.mycompany.com")
                .support("- Open a support request by email to support@mycompany.com"),
        custom_message);

    println!("A normal log message");
    panic!("OMG EVERYTHING IS ON FIRE!!!");
}
