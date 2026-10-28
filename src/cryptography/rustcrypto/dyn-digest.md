## Choose a hash algorithm at runtime

[![sha2-badge]][sha2] [![data-encoding-badge]][data-encoding] [![cat-cryptography-badge]][cat-cryptography]

Use [`DynDigest`] when the algorithm name comes from a config file, a command
line flag or a protocol field and is not known at compile time; this is the
job of `EVP_get_digestbyname` in OpenSSL. Every hash in the RustCrypto family
can be boxed as a `dyn DynDigest`, so `hasher_by_name` returns one value type
for all of them. Unsupported names return `None` instead of a null pointer.

```rust
{{#include ../../../crates/cryptography/rustcrypto/src/bin/dyn_digest.rs::27 }}
```

[`DynDigest`]: https://docs.rs/digest/*/digest/trait.DynDigest.html
