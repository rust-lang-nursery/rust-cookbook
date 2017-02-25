#Byteorder
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

<!-- Links -->

[byteorder-badge]: https://img.shields.io/crates/v/rustc-serialize.svg?label=byteorder
[byteorder]: https://docs.rs/byteorder
