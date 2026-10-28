## Upgrade stored password hashes on login

[![argon2-badge]][argon2] [![cat-cryptography-badge]][cat-cryptography]

Use this when you raise the cost of password hashing but cannot rehash users
who are not logged in. A PHC string records its own parameters, so
`verify_and_upgrade` can verify an old hash with the parameters stored in it,
read them back with [`Params::try_from`] and, if they are weaker than the
current ones, return a new hash to save. It returns `None` when the stored
hash is already current, and an error when the password is wrong.

```rust
{{#include ../../../crates/cryptography/rustcrypto/src/bin/rehash.rs::43 }}
```

[`Params::try_from`]: https://docs.rs/argon2/*/argon2/struct.Params.html
