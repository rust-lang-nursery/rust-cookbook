# A note about error handling

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
    let ref mut stdout = io::stdout();
    writeln!(stdout, "hello, world")?;

    Ok(())
}

fn main() { run().unwrap() }
```

and when necessary to reduce boilerplate,
they use the [error-chain] crate.

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

error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }
}

fn main() { run().unwrap() }
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

