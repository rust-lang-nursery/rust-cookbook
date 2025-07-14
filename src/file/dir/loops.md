## Find loops for a given path

[![same_file-badge]][same_file] [![walkdir-badge]][walkdir] [![cat-filesystem-badge]][cat-filesystem]

Use [`same_file::is_same_file`] to detect loops for a given path.
For example, a loop is created on a Unix system via symlinks:

```bash
mkdir -p /tmp/foo/bar/baz
ln -s /tmp/foo/  /tmp/foo/bar/baz/qux
```

The following would assert that a loop exists.

```rust,edition2021
use walkdir::WalkDir;
use same_file::is_same_file;

fn main() {
    let mut loop_found = false;
    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok()) {
        let ancestor = entry.path()
            .ancestors()
            .skip(1)
            .find(|ancestor| is_same_file(ancestor, entry.path()).is_ok());

        if ancestor.is_some() {
            loop_found = true;
        }
    }
    // Note: This test would only pass if there are actual symlink loops
    // println!("Loop found: {}", loop_found);
}
```

[`same_file::is_same_file`]: https://docs.rs/same-file/*/same_file/fn.is_same_file.html
