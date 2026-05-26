## Rename or atomically replace a file

[![std-badge]][std] [![tempfile-badge]][tempfile] [![cat-filesystem-badge]][cat-filesystem]

[`fs::rename`] is the atomic-replace primitive when source and destination
live on the same filesystem: on Unix it's a single `rename(2)` syscall, on
Windows it's `MoveFileExW` with `MOVEFILE_REPLACE_EXISTING`. A concurrent
reader either opens the old contents or the new — never a half-written file,
never a window where the path is missing.

The write-to-temp-then-rename pattern below is how editors, package managers,
and config writers avoid leaving a torn file behind if the process dies
mid-write. The staging file and the final path must live on the same
filesystem; across devices the kernel has no atomic move, and `rename` fails
instead of silently degrading to a copy.

```rust,edition2021
use std::fs;
use std::io;
use tempfile::tempdir;

fn main() -> io::Result<()> {
    let dir = tempdir()?;
    let final_path = dir.path().join("config.toml");
    let staging = dir.path().join("config.toml.new");

    // Write the new contents to a sibling path first...
    fs::write(&staging, "version = 2\n")?;

    // ...then swap it into place. Readers see either the old file or the
    // new one, never a partial write.
    fs::rename(&staging, &final_path)?;

    assert_eq!(fs::read_to_string(&final_path)?, "version = 2\n");
    assert!(!staging.exists());
    Ok(())
}
```

[`fs::rename`]: https://doc.rust-lang.org/std/fs/fn.rename.html
