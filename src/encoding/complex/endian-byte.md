## Read and write integers in little-endian byte order

[![std-badge]][std] [![cat-encoding-badge]][cat-encoding]

Reversing the significant bytes of structured data may be necessary when
receiving information over the network, such that bytes received are from
another system. The standard library covers this without any extra crate.

Every integer type has methods that convert to and from a byte array in a
chosen order: [`u16::to_le_bytes`] and [`u16::from_le_bytes`] for
little-endian, [`u16::to_be_bytes`] and [`u16::from_be_bytes`] for big-endian,
and [`u16::to_ne_bytes`] for the endianness of the current platform. The
arrays have a fixed size, so no allocation is needed to convert a single
value.

A byte slice implements [`Read`], so [`Read::read_exact`] fills each array
from the front of the slice and advances it. If the input is too short, it
returns an [`UnexpectedEof`] error instead of panicking.

```rust,edition2021
use std::io::{Error, Read};

#[derive(Default, PartialEq, Debug)]
struct Payload {
    kind: u8,
    value: u16,
}

fn main() -> Result<(), Error> {
    let original_payload = Payload {
        kind: 1,
        value: 0x0203,
    };
    let encoded_bytes = encode(&original_payload);
    assert_eq!(encoded_bytes, [0x01, 0x03, 0x02]);
    let decoded_payload = decode(&encoded_bytes)?;
    assert_eq!(original_payload, decoded_payload);
    assert!(decode(&encoded_bytes[..2]).is_err());
    Ok(())
}

fn encode(payload: &Payload) -> Vec<u8> {
    let mut bytes = vec![payload.kind];
    bytes.extend_from_slice(&payload.value.to_le_bytes());
    bytes
}

fn decode(mut bytes: &[u8]) -> Result<Payload, Error> {
    let mut kind = [0; 1];
    bytes.read_exact(&mut kind)?;
    let mut value = [0; 2];
    bytes.read_exact(&mut value)?;
    Ok(Payload {
        kind: kind[0],
        value: u16::from_le_bytes(value),
    })
}
```

The [byteorder] crate remains useful for streams. Its `ReadBytesExt` and
`WriteBytesExt` traits read and write integers directly on any [`Read`] or
[`Write`] without a temporary array, and it can convert whole slices of
integers at once. For a handful of fixed-size fields, the standard library
methods above are enough.

[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
[`UnexpectedEof`]: https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.UnexpectedEof
[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
[`u16::from_be_bytes`]: https://doc.rust-lang.org/std/primitive.u16.html#method.from_be_bytes
[`u16::from_le_bytes`]: https://doc.rust-lang.org/std/primitive.u16.html#method.from_le_bytes
[`u16::to_be_bytes`]: https://doc.rust-lang.org/std/primitive.u16.html#method.to_be_bytes
[`u16::to_le_bytes`]: https://doc.rust-lang.org/std/primitive.u16.html#method.to_le_bytes
[`u16::to_ne_bytes`]: https://doc.rust-lang.org/std/primitive.u16.html#method.to_ne_bytes
[byteorder]: https://docs.rs/byteorder/
