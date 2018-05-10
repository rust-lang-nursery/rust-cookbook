[ex-hex-encode-decode]: #ex-hex-encode-decode
<a name="ex-hex-encode-decode"></a>
## Encode and decode hex

[![data-encoding-badge]][data-encoding] [![cat-encoding-badge]][cat-encoding]

The [`data_encoding`] crate provides a `HEXUPPER::encode` method which
takes a `&[u8]` and returns a `String` containing the hexadecimal
representation of the data.

Similarly, a `HEXUPPER::decode` method is provided which takes a `&[u8]` and
returns a `Vec<u8>` if the input data is successfully decoded.

The example below shows a `&[u8]` of data being converted to its hexadecimal
representation and then being compared to its expected value. The returned
hex `String` is then converted back to its original representation and is
compared to the original value provided.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate data_encoding;

use data_encoding::{HEXUPPER, DecodeError};
#
# error_chain! {
#     foreign_links {
#         Decode(DecodeError);
#     }
# }

fn run() -> Result<()> {
    let original = b"The quick brown fox jumps over the lazy dog.";
    let expected = "54686520717569636B2062726F776E20666F78206A756D7073206F76\
        657220746865206C617A7920646F672E";

    let encoded = HEXUPPER.encode(original);
    assert_eq!(encoded, expected);

    let decoded = HEXUPPER.decode(&encoded.into_bytes())?;
    assert_eq!(&decoded[..], &original[..]);

    Ok(())
}
#
# quick_main!(run);
```

[`data_encoding`]: https://github.com/ia0/data-encoding
