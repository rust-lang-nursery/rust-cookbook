## Atomically replace a file with a temporary file

[![tempfile-badge]][tempfile] [![cat-filesystem-badge]][cat-filesystem]

[`NamedTempFile::persist`] is the [`tempfile`]-native form of the
write-to-temp-then-rename pattern. The [Rename or atomically replace a
file](#rename-or-atomically-replace-a-file) recipe shows the plain `std`
version. Creating the staging file with [`new_in`] on the destination directory
keeps it on the same filesystem. The final move is then an atomic rename rather
than a cross-device copy. [`persist`] performs that rename. It returns the
now-permanent [`File`].

A reader of the target path sees either the old contents or the complete new
ones. It never sees a half-written file. If the process dies before `persist`,
only the temporary file is left behind. The original is untouched.

```rust,edition2021
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let final_path = dir.path().join("config.toml");

    let mut staging = NamedTempFile::new_in(&dir)?;
    writeln!(staging, "version = 2")?;
    staging.flush()?;

    staging.persist(&final_path)?;

    assert_eq!(fs::read_to_string(&final_path)?, "version = 2\n");
    Ok(())
}
```

[`tempfile`]: https://docs.rs/tempfile/
[`new_in`]: https://docs.rs/tempfile/*/tempfile/struct.NamedTempFile.html#method.new_in
[`NamedTempFile::persist`]: https://docs.rs/tempfile/*/tempfile/struct.NamedTempFile.html#method.persist
[`persist`]: https://docs.rs/tempfile/*/tempfile/struct.NamedTempFile.html#method.persist
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
