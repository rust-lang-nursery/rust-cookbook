## Compress a file with gzip

[![flate2-badge]][flate2] [![cat-compression-badge]][cat-compression]

Compresses `input.txt` into `input.txt.gz` in the current working directory.

Wraps a [`File`] in a [`GzEncoder`] configured with a [`Compression`] level,
then copies the contents of the source file into the encoder with
[`io::copy`], transparently compressing the data as it is written to
`input.txt.gz`.

```rust,edition2018,no_run
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut input = File::open("input.txt")?;
    let output = File::create("input.txt.gz")?;
    let mut encoder = GzEncoder::new(output, Compression::default());
    io::copy(&mut input, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}
```

[`Compression`]: https://docs.rs/flate2/*/flate2/struct.Compression.html
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`GzEncoder`]: https://docs.rs/flate2/*/flate2/write/struct.GzEncoder.html
[`io::copy`]: https://doc.rust-lang.org/std/io/fn.copy.html
