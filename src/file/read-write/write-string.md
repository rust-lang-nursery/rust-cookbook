## Write a string to a file

[![std-badge]][std] [![tempfile-badge]][tempfile] [![cat-filesystem-badge]][cat-filesystem]

[`fs::write`] is the mirror of [`fs::read_to_string`]: it opens the path,
writes the bytes, and closes the file in one call. The semantics are
create-or-truncate — if the file exists, its previous contents are discarded;
if it doesn't, it's created with default permissions.

To append instead of replacing, build a [`File`] through [`OpenOptions`] with
[`append`] set. To refuse to overwrite an existing file — useful when a stale
file would be a bug — use [`create_new`] instead of `create`.

```rust,edition2021
use std::fs;
use std::io;
use tempfile::tempdir;

fn main() -> io::Result<()> {
    let dir = tempdir()?;
    let path = dir.path().join("report.txt");

    // Creates the file if missing.
    fs::write(&path, "build ok\n")?;

    // Calling write again replaces — does not append to — the previous contents.
    fs::write(&path, "build ok\nemitted 12 warnings\n")?;

    let back = fs::read_to_string(&path)?;
    assert_eq!(back, "build ok\nemitted 12 warnings\n");
    Ok(())
}
```

[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`OpenOptions`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
[`append`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.append
[`create_new`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create_new
[`fs::read_to_string`]: https://doc.rust-lang.org/std/fs/fn.read_to_string.html
[`fs::write`]: https://doc.rust-lang.org/std/fs/fn.write.html
