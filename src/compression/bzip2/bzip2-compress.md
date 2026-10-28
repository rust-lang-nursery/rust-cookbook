## Compress a file with bzip2

[![bzip2-badge]][bzip2] [![cat-compression-badge]][cat-compression]

Compresses `input.txt` into `input.txt.bz2` in the current working directory.

Wraps a [`File`] in a [`BzEncoder`] configured with a [`Compression`] level,
then copies the contents of the source file into the encoder with
[`io::copy`], transparently compressing the data as it is written to
`input.txt.bz2`.

```rust,edition2024,no_run
{{#include ../../../crates/compression/bzip2/src/bin/compress.rs::13 }}
```

[`BzEncoder`]: https://docs.rs/bzip2/*/bzip2/write/struct.BzEncoder.html
[`Compression`]: https://docs.rs/bzip2/*/bzip2/struct.Compression.html
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`io::copy`]: https://doc.rust-lang.org/std/io/fn.copy.html
