# Extensible RustCrypto

The RustCrypto crates share traits such as [`Digest`], [`DynDigest`] and
[`Mac`], so you can write code that works with any algorithm, or picks one at
runtime. These recipes cover jobs that are awkward with a C-style API, where
the algorithm is fixed by a function name; see
[Choosing between ring and RustCrypto] for when to use them. The simple
one-shot versions are on the [Hashing](hashing.html) page.

{{#include rustcrypto/dyn-digest.md}}

{{#include rustcrypto/tee-hash.md}}

{{#include rustcrypto/rehash.md}}

{{#include rustcrypto/generic-mac.md}}

[Choosing between ring and RustCrypto]: hashing.html#choosing-between-ring-and-rustcrypto
[`Digest`]: https://docs.rs/digest/*/digest/trait.Digest.html
[`DynDigest`]: https://docs.rs/digest/*/digest/trait.DynDigest.html
[`Mac`]: https://docs.rs/digest/*/digest/trait.Mac.html

{{#include ../links.md}}
