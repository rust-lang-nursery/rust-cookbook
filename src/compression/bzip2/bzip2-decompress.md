## Decompress a bzip2 file

[![bzip2-badge]][bzip2] [![cat-compression-badge]][cat-compression]

Decompresses a single bzip2-compressed file `input.txt.bz2` into `input.txt`
in the current working directory.

Wraps a [`File`] in a [`BzDecoder`] and copies the decompressed bytes into the
destination file with [`io::copy`].

```rust,edition2024,no_run
{{#include ../../../crates/compression/bzip2/src/bin/decompress.rs::11 }}
```

[`BzDecoder`]: https://docs.rs/bzip2/*/bzip2/read/struct.BzDecoder.html
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`io::copy`]: https://doc.rust-lang.org/std/io/fn.copy.html
