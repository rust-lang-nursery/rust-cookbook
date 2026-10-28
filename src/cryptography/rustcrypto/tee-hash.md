## Hash data while writing it

[![sha2-badge]][sha2] [![data-encoding-badge]][data-encoding] [![cat-cryptography-badge]][cat-cryptography]

Use a wrapper around [`Write`] when you want the digest of a stream you are
already writing, such as a download or an archive, without reading it back.
`HashingWriter` works with any [`Digest`], passes writes through to the inner
writer and hashes only the bytes it accepted, so partial writes stay correct.
Because it is a `Write`, it plugs into [`io::copy`] and anything else that
takes a writer.

```rust
{{#include ../../../crates/cryptography/rustcrypto/src/bin/tee_hash.rs::55 }}
```

[`Digest`]: https://docs.rs/digest/*/digest/trait.Digest.html
[`io::copy`]: https://doc.rust-lang.org/std/io/fn.copy.html
[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
