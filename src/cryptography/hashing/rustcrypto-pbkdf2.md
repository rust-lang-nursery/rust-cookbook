## Salt and hash a password with PBKDF2 using pbkdf2

[![pbkdf2-badge]][pbkdf2] [![cat-cryptography-badge]][cat-cryptography]

Use this form when you want to store a single string per user; see
[Choosing between ring and RustCrypto]. [`Pbkdf2::hash_password`] generates a
random salt and returns a PHC string such as
`$pbkdf2-sha256$i=600000,l=32$salt$hash`, which holds the algorithm, iteration
count, salt and hash together. [`Pbkdf2::verify_password`] reads those
parameters back out of the string, so the iteration count can be raised later
without breaking old hashes.

> This recipe requires the [`phc`] feature flag of `pbkdf2` to be enabled in `Cargo.toml`.

```rust
{{#include ../../../crates/cryptography/rustcrypto/src/bin/pbkdf2.rs::28 }}
```

[`Pbkdf2::hash_password`]: https://docs.rs/pbkdf2/*/pbkdf2/struct.Pbkdf2.html
[`Pbkdf2::verify_password`]: https://docs.rs/pbkdf2/*/pbkdf2/struct.Pbkdf2.html
[`phc`]: https://docs.rs/crate/pbkdf2/*/features#phc
