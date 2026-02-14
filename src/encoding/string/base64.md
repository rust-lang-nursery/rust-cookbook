## Encode and decode base64

[![base64-badge]][base64] [![cat-encoding-badge]][cat-encoding]

Encodes byte slice into `base64` String using [`encode`]
and decodes it with [`decode`].

```rust,edition2018
use anyhow::Result;
use std::str;
use base64::prelude::{Engine as _, BASE64_STANDARD};

fn main() -> Result<()> {
    let hello = b"hello rustaceans";
    let encoded = BASE64_STANDARD.encode(hello);
    let decoded = BASE64_STANDARD.decode(&encoded)?;

    println!("origin: {}", str::from_utf8(hello)?);
    println!("base64 encoded: {}", encoded);
    println!("back to origin: {}", str::from_utf8(&decoded)?);

    Ok(())
}
```

[`decode`]: https://docs.rs/base64/latest/base64/engine/trait.Engine.html#method.decode
[`encode`]: https://docs.rs/base64/latest/base64/engine/trait.Engine.html#method.encode
