# About "Cookin' with Rust"

## Who this book is for

## How to read this book

## How to use the examples

## A note about error handling

Error handling in Rust is robust when done correctly, but in today's
Rust it requires a fair bit of boilerplate. Because of this one often
sees Rust examples filled with `unwrap` calls instead of proper error
handling.

Since these recipes are intended to be reused as-is and encourage best
practices, they set up error handling correctly when there are
`Result` types involved.

The basic pattern we use is to have a `fn run() -> Result` that acts
like the "real" main function.

The code for this setup generally looks like:

```rust
use std::io::{self, Write};

fn run() -> io::Result<()> {
    writeln!(io::stderr(), "hello, world")?;

    Ok(())
}

fn main() {
    run().unwrap();
}
```

and when necessary to reduce boilerplate,
they use the [error-chain] crate.

```rust
use std::net::IpAddr;
use std::str;

#[macro_use]
extern crate error_chain;

error_chain! {
    foreign_links {
        Utf8(std::str::Utf8Error);
        AddrParse(std::net::AddrParseError);
    }
}

fn run() -> Result<()> {
    let bytes = b"2001:db8::1";

    // Bytes to string.
    let s = str::from_utf8(bytes)?;

    // String to IP address.
    let addr: IpAddr = s.parse()?;

    println!("{:?}", addr);
    Ok(())
}

fn main() {
    run().unwrap();
}
```

This is using the `error_chain!` macro to define a custom `Error`
and `Result` type, along with an automatic conversion from
the common `::std::io::Error` type. The automatic conversion
makes the `?` operator work .

For more background on error handling in Rust, read [this page of the
Rust book][error-docs] and [this blog post][error-blog].

<!-- Links-->

[error-docs]: https://doc.rust-lang.org/book/error-handling.html
[error-blog]: https://brson.github.io/2016/11/30/starting-with-error-chain
[error-chain]: https://docs.rs/error-chain/
