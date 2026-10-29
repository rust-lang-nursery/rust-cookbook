## Sign and verify a message with HMAC using hmac and sha2

[![hmac-badge]][hmac] [![sha2-badge]][sha2] [![cat-cryptography-badge]][cat-cryptography]

Use `hmac` when the hash function should be chosen with a type alias; see
[Choosing between ring and RustCrypto]. [`Hmac`] wraps any hash from the
RustCrypto family, so `type HmacSha256 = Hmac<Sha256>` becomes SHA-512 by
changing one word. [`Mac::finalize`] computes the tag and
[`Mac::verify_slice`] compares a tag in constant time.

```rust
{{#include ../../../crates/cryptography/rustcrypto/src/bin/hmac.rs::27 }}
```

[`Hmac`]: https://docs.rs/hmac/*/hmac/struct.Hmac.html
[`Mac::finalize`]: https://docs.rs/digest/*/digest/trait.Mac.html#tymethod.finalize
[`Mac::verify_slice`]: https://docs.rs/digest/*/digest/trait.Mac.html#method.verify_slice
<!-- needs-no-features: getrandom -->
