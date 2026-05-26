## Create a nested directory tree

[![std-badge]][std] [![tempfile-badge]][tempfile] [![cat-filesystem-badge]][cat-filesystem]

[`fs::create_dir_all`] is `mkdir -p`: it walks the requested path and creates
every missing intermediate directory. It's idempotent — calling it on a path
that already exists returns `Ok(())`, not an error — so it's the right call
when seeding cache directories or log roots on startup.

[`fs::create_dir`] only creates the final component. If any parent on the way
is missing it fails with [`io::ErrorKind::NotFound`]. Reach for `create_dir`
when the parent is known to exist and you want the operation to fail loudly
if your assumption is wrong.

```rust,edition2021
use std::fs;
use std::io;
use tempfile::tempdir;

fn main() -> io::Result<()> {
    let root = tempdir()?;
    let nested = root.path().join("cache").join("2026").join("05");

    // Creates every missing component along the way.
    fs::create_dir_all(&nested)?;
    assert!(nested.is_dir());

    // Calling it again on an existing directory is not an error.
    fs::create_dir_all(&nested)?;

    // `create_dir` only creates the leaf, and only when the parent exists.
    let leaf = nested.join("26");
    fs::create_dir(&leaf)?;
    assert!(leaf.is_dir());

    Ok(())
}
```

[`fs::create_dir`]: https://doc.rust-lang.org/std/fs/fn.create_dir.html
[`fs::create_dir_all`]: https://doc.rust-lang.org/std/fs/fn.create_dir_all.html
[`io::ErrorKind::NotFound`]: https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.NotFound
