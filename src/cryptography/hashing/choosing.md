## Choosing between ring and RustCrypto
<!-- not-a-recipe -->

- [`ring`] is a single crate with a fixed set of algorithms, built on C and
  assembly code from BoringSSL. Choose it when SHA-2, HMAC and PBKDF2 are all
  you need and you want one dependency with one API.
- [RustCrypto] is a family of separate crates ([`sha2`], [`hmac`], [`pbkdf2`],
  [`argon2`]) written in Rust that share the traits of the [`digest`] crate, so
  the hash function is a type parameter. Choose it when you want generic code,
  algorithms `ring` lacks such as Argon2, or a build without C.
- Both are pre-1.0, so their APIs may still change.

The RustCrypto versions of the recipes follow the `ring` ones. For patterns that
use the shared traits, see [Extensible RustCrypto](extensible.html).

[`argon2`]: https://docs.rs/argon2/
[`digest`]: https://docs.rs/digest/
[`hmac`]: https://docs.rs/hmac/
[`pbkdf2`]: https://docs.rs/pbkdf2/
[`ring`]: https://docs.rs/ring/
[RustCrypto]: https://github.com/RustCrypto
[`sha2`]: https://docs.rs/sha2/
