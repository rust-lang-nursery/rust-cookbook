## Encode and decode base64

[![base64-badge]][base64] [![cat-encoding-badge]][cat-encoding]

Encodes byte slice into `base64` String using [`encode`]
and decodes it with [`decode`].

```rust
{{#include ../../../crates/encoding/base64/src/bin/base64.rs }}
```

[`decode`]: https://docs.rs/base64/latest/base64/engine/trait.Engine.html#method.decode
[`encode`]: https://docs.rs/base64/latest/base64/engine/trait.Engine.html#method.encode
