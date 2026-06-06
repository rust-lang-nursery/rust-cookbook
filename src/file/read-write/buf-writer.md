## Write to a file with BufWriter

[![std-badge]][std] [![tempfile-badge]][tempfile] [![cat-filesystem-badge]][cat-filesystem]

[`BufWriter`] wraps any [`io::Write`] and batches small writes into a single
in-memory buffer — typically 8 KB — before flushing to the OS. This reduces
syscall overhead when making many small writes. Always call [`Write::flush`]
explicitly when you are done; the buffer is also flushed when the `BufWriter`
is dropped, but any error from that implicit flush is silently discarded.

```rust,edition2021
use std::fs::File;
use std::io::{self, BufWriter, Write};
use tempfile::tempdir;

fn main() -> io::Result<()> {
    let dir = tempdir()?;
    let path = dir.path().join("report.txt");

    let file = File::create(&path)?;
    let mut writer = BufWriter::new(file);

    // These writes go into an in-memory buffer, not directly to disk.
    writeln!(writer, "line one")?;
    writeln!(writer, "line two")?;
    writeln!(writer, "line three")?;

    // Explicit flush drains the buffer to the OS in one syscall.
    writer.flush()?;
    Ok(())
}
```

[`BufWriter`]: https://doc.rust-lang.org/std/io/struct.BufWriter.html
[`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
[`Write::flush`]: https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush
