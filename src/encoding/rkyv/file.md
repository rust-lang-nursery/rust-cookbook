## Read an archive from a file

[![rkyv-badge]][rkyv] [![cat-encoding-badge]][cat-encoding]

Writes a `Vec<Reading>` to `readings.rkyv` in the current working directory,
then reads it back and iterates over the archived readings in place.

[`rkyv::access`] requires the buffer to be suitably aligned for the archived
types. [`fs::read`] returns a `Vec<u8>` with no alignment guarantee, so the
bytes are copied into an [`AlignedVec`] before access. The archived type of
`Vec<Reading>` is named with [`rkyv::Archived`].

```rust,no_run,noplayground
{{#include ../../../crates/encoding/rkyv/src/bin/file.rs::36 }}
```

[`AlignedVec`]: https://docs.rs/rkyv/*/rkyv/util/struct.AlignedVec.html
[`fs::read`]: https://doc.rust-lang.org/std/fs/fn.read.html
[`rkyv::access`]: https://docs.rs/rkyv/*/rkyv/fn.access.html
[`rkyv::Archived`]: https://docs.rs/rkyv/*/rkyv/type.Archived.html
