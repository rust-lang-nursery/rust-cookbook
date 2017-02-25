# Cookin' with Rust

A practical guide to the Rust crate ecosystem.

## Recipes
### [Byteorder](pages/byteorder.md) [![byteorder][byteorder-badge]][byteorder]
### [File IO](pages/fileio.md) [![file][file-badge]][file]
### [Command Line Parsing](pages/cliparsing.md) [![clap][clap-badge]][clap]
### [JSON](pages/json.md) [![json][json-badge]][json]
### [TOML](pages/toml.md) [![toml][toml-badge]][toml]


## A note about error handling

Error handling in Rust is robust when done correctly, but in today's
Rust it requires a fair bit of boilerplate. Because of this one often
sees Rust examples filled with `unwrap` calls instead of proper error
handling.

Since these recipes are intended to be reused as-is and encourage best
practices, they set up error handling correctly, and when necessary to
reduce boilerplate, they use the [error-chain] crate.

The code for this setup generally looks like:

```rust
#[macro_use]
extern crate error_chain;

mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
        }
    }
}

use errors::*;

fn main() { run().unwrap() }

fn run() -> Result<()> {
    use std::io::Write;
    let ref mut stdout = ::std::io::stdout();
    writeln!(stdout, "hello, world")?;

    Ok(())
}
```

This is using the `error_chain!` macro to define a custom `Error`
and `Result` type, along with an automatic conversion from
the common `::std::io::Error` type. The automatic conversion
makes the `?` operator work 

For more background on error handling in Rust, read [this page of the Rust book][error-docs] and [this blog post][error-blog].

## Contributing
If you'd like to make changes to the project, please see [this guide](CONTRIBUTING.md).

## License

MIT/Apache-2.0


<!-- Links -->

[byteorder-badge]: https://img.shields.io/crates/v/rustc-serialize.svg?label=byteorder
[byteorder]: https://docs.rs/byteorder
[file-badge]: https://img.shields.io/crates/v/rustc-serialize.svg?label=file
[file]: https://doc.rust-lang.org/std/fs/struct.File.html
[clap-badge]: https://img.shields.io/crates/v/rustc-serialize.svg?label=clap
[clap]: https://kbknapp.github.io/clap-rs/clap/struct.Arg.html
[json-badge]: https://img.shields.io/crates/v/rustc-serialize.svg?label=json
[json]: http://json.rs/doc/json
[error-docs]: https://doc.rust-lang.org/book/error-handling.html 
[error-blog]: https://brson.github.io/2016/11/30/starting-with-error-chain
