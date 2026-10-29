## Salt and hash a password with Argon2

[![argon2-badge]][argon2] [![cat-cryptography-badge]][cat-cryptography]

Use Argon2 for new password storage unless you need a FIPS-approved algorithm;
see [Choosing between ring and RustCrypto]. `ring` has no Argon2.
[`Argon2::default`] selects Argon2id with the crate's default memory and time
cost. [`Argon2::hash_password`] generates a random salt and returns a PHC string
that includes those parameters, and [`Argon2::verify_password`] checks a
password against it.

```rust
{{#include ../../../crates/cryptography/rustcrypto/src/bin/argon2.rs::28 }}
```

[`Argon2::default`]: https://docs.rs/argon2/*/argon2/struct.Argon2.html
[`Argon2::hash_password`]: https://docs.rs/argon2/*/argon2/struct.Argon2.html
[`Argon2::verify_password`]: https://docs.rs/argon2/*/argon2/struct.Argon2.html
