## Create a temporary file

[![tempfile-badge]][tempfile] [![cat-filesystem-badge]][cat-filesystem]

A [`NamedTempFile`] is a real, named file in the system temporary directory. It
deletes itself when its handle drops. That includes an early return through `?`,
so scratch files never leak. The file is created atomically and held open,
unlike a path assembled from a guessed name. Its [`path`] can be handed to other
code while the handle keeps it alive.

The handle implements [`Read`], [`Write`] and [`Seek`] just like any other
[`File`]. Seeking back to the start reads back what was just written.

```rust,edition2021
use std::io::{Read, Seek, SeekFrom, Write};
use tempfile::NamedTempFile;

fn main() -> std::io::Result<()> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "scratch data")?;

    file.seek(SeekFrom::Start(0))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    assert_eq!(contents, "scratch data\n");
    Ok(())
}
```

[`NamedTempFile`]: https://docs.rs/tempfile/*/tempfile/struct.NamedTempFile.html
[`path`]: https://docs.rs/tempfile/*/tempfile/struct.NamedTempFile.html#method.path
[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
[`Seek`]: https://doc.rust-lang.org/std/io/trait.Seek.html
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
