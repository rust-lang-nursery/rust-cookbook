## Handle errors correctly in main

[![error-chain-badge]][error-chain] [![cat-rust-patterns-badge]][cat-rust-patterns]

Handles error that occur when trying to open a file that does not
exist. It is achieved by using [error-chain], a library that takes
care of a lot of boilerplate code needed in order to [handle errors in Rust].

`Io(std::io::Error)` inside [`foreign_links`] allows automatic
conversion from [`std::io::Error`] into [`error_chain!`] defined type
implementing the [`Error`] trait.

The below recipe will tell how long the system has been running by
opening the Unix file `/proc/uptime` and parse the content to get the
first number. Returns uptime unless there is an error.

Other recipes in this book will hide the [error-chain] boilerplate, and can be
seen by expanding the code with the ⤢ button.

```rust
#[macro_use]
extern crate error_chain;

use std::fs::File;
use std::io::Read;

error_chain!{
    foreign_links {
        Io(std::io::Error);
        ParseInt(::std::num::ParseIntError);
    }
}

fn read_uptime() -> Result<u64> {
    let mut uptime = String::new();
    File::open("/proc/uptime")?.read_to_string(&mut uptime)?;

    Ok(uptime
        .split('.')
        .next()
        .ok_or("Cannot parse uptime data")?
        .parse()?)
}

fn main() {
    match read_uptime() {
        Ok(uptime) => println!("uptime: {} seconds", uptime),
        Err(err) => eprintln!("error: {}", err),
    };
}
```

[`error_chain!`]: https://docs.rs/error-chain/*/error_chain/macro.error_chain.html
[`Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
[`foreign_links`]: https://docs.rs/error-chain/*/error_chain/#foreign-links
[`std::io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html

[handle errors in Rust]: https://doc.rust-lang.org/book/second-edition/ch09-00-error-handling.html
