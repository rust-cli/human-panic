# human-panic
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Panic messages for humans. Replacement for
[`std::panic::catch_unwind`](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html)
to make errors nice for humans.

- [Documentation][8]
- [Crates.io][2]

## Why?
When you're building a CLI, polish is super important. Even though Rust is
pretty great at safety, it's not unheard of to access the wrong index in a
vector or have an assert fail somewhere.

When an error eventually occurs, you probably will want to know about it. So
instead of just providing an error message on the command line, we can create a
call to action for people to submit a report.

This should empower people to engage in communication, lowering the chances
people might get frustrated. And making it easier to figure out what might be
causing bugs.

### Default Output

```txt
thread 'main' panicked at 'oops', examples/main.rs:2:3
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

### Human-Panic Output

```txt
Well, this is embarrassing.

human-panic had a problem and crashed. To help us diagnose the problem you can send us a crash report.

We have generated a report file at "/var/folders/zw/bpfvmq390lv2c6gn_6byyv0w0000gn/T/report-8351cad6-d2b5-4fe8-accd-1fcbf4538792.toml". Submit an issue or email with the subject of "human-panic Crash Report" and include the report as an attachment.

- Homepage: https://github.com/yoshuawuyts/human-panic
- Authors: Yoshua Wuyts <yoshuawuyts@gmail.com>

We take privacy seriously, and do not perform any automated error collection. In order to improve the software, we rely on people to submit reports.

Thank you kindly!
```

## Usage

```rust no_run
#[macro_use]
extern crate human_panic;

fn main() {
   setup_panic!();

   println!("A normal log message");
   panic!("OMG EVERYTHING IS ON FIRE!!!")
}
```

## Installation

```sh
$ cargo add human-panic
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/human-panic.svg?style=flat-square
[2]: https://crates.io/crates/human-panic
[3]: https://img.shields.io/travis/yoshuawuyts/human-panic.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/human-panic
[5]: https://img.shields.io/crates/d/human-panic.svg?style=flat-square
[6]: https://crates.io/crates/human-panic
[7]: https://docs.rs/human-panic/badge.svg
[8]: https://docs.rs/human-panic
