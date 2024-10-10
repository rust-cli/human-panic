use human_panic::setup_panic;

fn main() {
    setup_panic!();

    call_inline_always()
}

fn call_inline_always() {
    inline_always();
}

#[inline(always)]
fn inline_always() {
    call_closure();
}

fn call_closure() {
    let closure = || {
        do_panic();
    };
    closure();
}

fn do_panic() {
    println!("A normal log message");
    panic!("OMG EVERYTHING IS ON FIRE!!!");
}
