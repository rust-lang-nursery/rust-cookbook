## Read a whole file into a string

[![std-badge]][std] [![tempfile-badge]][tempfile] [![cat-filesystem-badge]][cat-filesystem]

[`fs::read_to_string`] opens the file, reads it to end, closes it, and returns
a [`String`] — one call for the common case of loading a small text file in
full. It sizes the buffer from the file's length up front, so the read is a
single allocation plus a single syscall loop.

For files large enough that loading them whole would waste memory, switch to a
[`BufReader`] over [`File::open`] and stream lines or bytes. The cutoff is
workload-specific; past a few megabytes, the streaming form is usually the
right choice.

```rust,edition2021
use std::fs;
use std::io;
use tempfile::tempdir;

fn main() -> io::Result<()> {
    let dir = tempdir()?;
    let path = dir.path().join("config.toml");
    fs::write(&path, "name = \"cookbook\"\nedition = 2021\n")?;

    // One call: open, read, close.
    let contents = fs::read_to_string(&path)?;
    assert!(contents.contains("cookbook"));
    println!("{contents}");
    Ok(())
}
```

[`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
[`File::open`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.open
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
[`fs::read_to_string`]: https://doc.rust-lang.org/std/fs/fn.read_to_string.html
