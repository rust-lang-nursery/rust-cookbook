## Find loops for a given path

[![same_file-badge]][same_file] [![cat-filesystem-badge]][cat-filesystem]

Use [`same_file::is_same_file`] to detect loops for a given path.
For example, a loop could be created on a Unix system via symlinks:
```bash
mkdir -p /tmp/foo/bar/baz
ln -s /tmp/foo/  /tmp/foo/bar/baz/qux
```
The following would assert that a loop exists.

```rust,no_run
extern crate same_file;

use std::io;
use std::path::{Path, PathBuf};
use same_file::is_same_file;

fn contains_loop<P: AsRef<Path>>(path: P) -> io::Result<Option<(PathBuf, PathBuf)>> {
    let path = path.as_ref();
    let mut path_buf = path.to_path_buf();
    while path_buf.pop() {
        if is_same_file(&path_buf, path)? {
            return Ok(Some((path_buf, path.to_path_buf())));
        } else if let Some(looped_paths) = contains_loop(&path_buf)? {
            return Ok(Some(looped_paths));
        }
    }
    return Ok(None);
}

fn main() {
    assert_eq!(
        contains_loop("/tmp/foo/bar/baz/qux/bar/baz").unwrap(),
        Some((
            PathBuf::from("/tmp/foo"),
            PathBuf::from("/tmp/foo/bar/baz/qux")
        ))
    );
}
```

[`same_file::is_same_file`]: https://docs.rs/same-file/*/same_file/fn.is_same_file.html
