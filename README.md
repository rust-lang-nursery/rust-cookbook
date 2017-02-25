# Cookin' with Rust

A practical guide to the Rust crate ecosystem.

- [Read and write integers in little-endian byte order](#byteorder)
  [![byteorder][byteorder-badge]][byteorder]


# A note about error handling

Error handling in Rust is robust when done correctly, but in today's
Rust it requires a fair bit of boilerplate. Because of this one often
seees Rust examples filled with `unwrap` calls instead of proper error
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

For more background on error handling in Rust, read TODO and TODO.


<a id="byteorder"></a>
## Read and write integers in little-endian byte order

[![byteorder][byteorder-badge]][byteorder]

```rust
extern crate byteorder;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[derive(Default, Eq, PartialEq, Debug)]
struct Payload {
  kind: u8,
  value: u16,
}

fn run() -> Result<()> {
   let original_payload = Payload::default();
   let encoded_buf = encode(&original_payload)?;
   let decoded_payload = decode(&encoded_buf)?;
   assert_eq!(original_payload, decoded_payload);
   Ok(())
}

fn encode(payload: &Payload) -> Result<Vec<u8>> {
   let mut wtr = vec![];
   wtr.write_u8(payload.kind)?;
   wtr.write_u16::<LittleEndian>(payload.value)?;
   Ok(wtr)
}

fn decode(buf: &[u8]) -> Result<Payload> {
    let mut rdr = Cursor::new(buf);
    Ok(Payload {
        kind: rdr.read_u8()?,
        value: rdr.read_u16::<LittleEndian>()?,
    })
}

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
```


# License

MIT/Apache-2.0


<!-- Links -->

[byteorder-badge]: https://img.shields.io/crates/v/rustc-serialize.svg
[byteorder]: https://docs.rs/byteorder
