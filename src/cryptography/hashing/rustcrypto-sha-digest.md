## Calculate the SHA-256 digest of a file with sha2

[![sha2-badge]][sha2] [![data-encoding-badge]][data-encoding] [![cat-cryptography-badge]][cat-cryptography]

Use `sha2` when you want the hash function to be a type parameter; see
[Choosing between ring and RustCrypto]. `digest_reader` works with any hash that
implements [`Digest`], so moving from [`Sha256`] to [`Sha512`] only changes
the type argument. Data is fed to the hasher with [`Digest::update`] and the
result is returned by [`Digest::finalize`].

```rust
{{#include ../../../crates/cryptography/rustcrypto/src/bin/sha256_file.rs::38 }}
```

[`Digest`]: https://docs.rs/digest/*/digest/trait.Digest.html
[`Digest::finalize`]: https://docs.rs/digest/*/digest/trait.Digest.html#tymethod.finalize
[`Digest::update`]: https://docs.rs/digest/*/digest/trait.Digest.html#tymethod.update
[`Sha256`]: https://docs.rs/sha2/*/sha2/type.Sha256.html
[`Sha512`]: https://docs.rs/sha2/*/sha2/type.Sha512.html
