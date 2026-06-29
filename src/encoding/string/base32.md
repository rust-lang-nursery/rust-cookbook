## Encode and decode base32

[![data-encoding-badge]][data-encoding] [![cat-encoding-badge]][cat-encoding]

The [`data_encoding`] crate provides a `BASE32::encode` method which takes a
`&[u8]` and returns a `String` containing the base32 representation of the data.

Similarly, a `BASE32::decode` method is provided which takes a `&[u8]` and
returns a `Vec<u8>` if the input data is successfully decoded.

The example below coverts `&[u8]` data to base32 equivalent and compares this
value to the expected value.

```rust,edition2018
use data_encoding::{BASE32, DecodeError};

fn main() -> Result<(), DecodeError> {
    let original = b"Cooking with Rust";
    let expected = "INXW623JNZTSA53JORUCAUTVON2A====";

    let encoded = BASE32.encode(original);
    assert_eq!(encoded, expected);

    let decoded = BASE32.decode(encoded.as_bytes())?;
    assert_eq!(&decoded[..], &original[..]);

    Ok(())
}
```

[`data_encoding`]: https://docs.rs/data-encoding/*/data_encoding/
