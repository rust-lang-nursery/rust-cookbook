## Access serialized data without deserializing it

[![rkyv-badge]][rkyv] [![cat-encoding-badge]][cat-encoding]

[`rkyv`] lays out serialized bytes so that the buffer itself is a valid,
aligned in-memory representation of the data. Deriving [`Archive`] on
`Player` generates an `ArchivedPlayer` type whose fields are archived
counterparts such as [`ArchivedString`] and [`ArchivedVec`].

[`rkyv::to_bytes`] serializes the value into an [`AlignedVec`].
[`rkyv::access`] validates the buffer and returns a `&ArchivedPlayer` that
points straight into it, so reading fields requires no parsing, copying, or
allocation. Archived integers are stored in a fixed byte order, so
[`to_native`] converts them to native integers for arithmetic. When an owned
value is needed, [`rkyv::deserialize`] converts the archive back into a
`Player`.

```rust,noplayground
{{#include ../../../crates/encoding/rkyv/src/bin/zero-copy.rs::31 }}
```

[`AlignedVec`]: https://docs.rs/rkyv/*/rkyv/util/struct.AlignedVec.html
[`Archive`]: https://docs.rs/rkyv/*/rkyv/trait.Archive.html
[`ArchivedString`]: https://docs.rs/rkyv/*/rkyv/string/struct.ArchivedString.html
[`ArchivedVec`]: https://docs.rs/rkyv/*/rkyv/vec/struct.ArchivedVec.html
[`rkyv`]: https://docs.rs/rkyv/
[`rkyv::access`]: https://docs.rs/rkyv/*/rkyv/fn.access.html
[`rkyv::deserialize`]: https://docs.rs/rkyv/*/rkyv/fn.deserialize.html
[`rkyv::to_bytes`]: https://docs.rs/rkyv/*/rkyv/fn.to_bytes.html
[`to_native`]: https://docs.rs/rend/*/rend/struct.u32_le.html#method.to_native
