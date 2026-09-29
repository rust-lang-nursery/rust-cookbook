## Decompress a gzip file

[![flate2-badge]][flate2] [![cat-compression-badge]][cat-compression]

Decompresses a single gzip-compressed file `input.txt.gz` into `input.txt` in
the current working directory. Unlike [decompressing a tarball][ex-tar-decompress],
this reads plain (non-archive) gzip data, such as a single log or text file.

Wraps a [`File`] in a [`GzDecoder`] and copies the decompressed bytes into the
destination file with [`io::copy`].

```rust,edition2018,no_run
use flate2::read::GzDecoder;
use std::fs::File;
use std::io;

fn main() -> Result<(), io::Error> {
    let input = File::open("input.txt.gz")?;
    let mut decoder = GzDecoder::new(input);
    let mut output = File::create("input.txt")?;
    io::copy(&mut decoder, &mut output)?;
    Ok(())
}
```

[ex-tar-decompress]: tar.html#decompress-a-tarball
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`GzDecoder`]: https://docs.rs/flate2/*/flate2/read/struct.GzDecoder.html
[`io::copy`]: https://doc.rust-lang.org/std/io/fn.copy.html
