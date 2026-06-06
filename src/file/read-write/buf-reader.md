## Read a file line by line with BufReader

[![std-badge]][std] [![tempfile-badge]][tempfile] [![cat-filesystem-badge]][cat-filesystem]

For files too large to load into memory at once, wrap a [`File`] in a
[`BufReader`] and iterate with [`BufRead::lines`]. Unlike
[`fs::read_to_string`], this streams one kernel-buffered chunk at a time —
typically 8 KB — so memory usage stays flat regardless of file size.

```rust,edition2021
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use tempfile::tempdir;

fn main() -> io::Result<()> {
    let dir = tempdir()?;
    let path = dir.path().join("large.txt");

    // Write some lines so the example is self-contained.
    let mut file = File::create(&path)?;
    writeln!(file, "line one")?;
    writeln!(file, "line two")?;
    writeln!(file, "line three")?;

    // BufReader wraps the File and adds an internal 8 KB buffer.
    let reader = BufReader::new(File::open(&path)?);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
```

[`BufRead::lines`]: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
[`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`fs::read_to_string`]: https://doc.rust-lang.org/std/fs/fn.read_to_string.html
