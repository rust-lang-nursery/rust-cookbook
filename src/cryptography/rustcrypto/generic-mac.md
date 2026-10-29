## Write code that is generic over the MAC

[![hmac-badge]][hmac] [![sha2-badge]][sha2] [![cat-cryptography-badge]][cat-cryptography]

Use the [`Mac`] trait when the algorithm should be chosen by the caller, for
example to migrate from HMAC-SHA256 to HMAC-SHA512. `sign` and `verify` take
the MAC as a type parameter, so there is one implementation for every
algorithm that implements `Mac` and [`KeyInit`]. [`Mac::verify_slice`] compares
in constant time. A tag only verifies with the algorithm that made it, which
the second `assert!` shows.

```rust
{{#include ../../../crates/cryptography/rustcrypto/src/bin/generic_mac.rs::37 }}
```

[`KeyInit`]: https://docs.rs/digest/*/digest/trait.KeyInit.html
[`Mac`]: https://docs.rs/digest/*/digest/trait.Mac.html
[`Mac::verify_slice`]: https://docs.rs/digest/*/digest/trait.Mac.html#method.verify_slice
