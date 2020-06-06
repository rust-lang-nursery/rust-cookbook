## Encode and decode base64

[![base64-badge]][base64] [![cat-encoding-badge]][cat-encoding]

Encodes byte slice into `base64` String using [`encode`]
and decodes it with [`decode`].

```rust,edition2018
# use error_chain::error_chain;

use std::str;
use base64::{encode, decode};
#
# error_chain! {
#     foreign_links {
#         Base64(base64::DecodeError);
#         Utf8Error(str::Utf8Error);
#     }
# }

fn main() -> Result<()> {
    let hello = b"hello rustaceans";
    let encoded = encode(hello);
    let decoded = decode(&encoded)?;

    println!("origin: {}", str::from_utf8(hello)?);
    println!("base64 encoded: {}", encoded);
    println!("back to origin: {}", str::from_utf8(&decoded)?);

    Ok(())
}
```

[`decode`]: https://docs.rs/base64/*/base64/fn.decode.html
[`encode`]: https://docs.rs/base64/*/base64/fn.encode.html
