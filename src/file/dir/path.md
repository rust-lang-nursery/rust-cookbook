## Construct and inspect a path

[![std-badge]][std] [![cat-filesystem-badge]][cat-filesystem]

Most filesystem code is path arithmetic before it touches the disk. [`PathBuf`]
owns the buffer; [`Path`] is the borrowed view. The inspection methods —
[`file_name`], [`file_stem`], [`extension`], [`parent`] — return `Option`
because a path might not have the component you ask for.

[`PathBuf::push`] grows the buffer one component at a time. [`with_extension`]
and [`with_file_name`] derive sibling or backup paths in one call without
mutating the original. Conversion to `&str` goes through [`Path::to_str`],
which returns `None` on non-UTF-8 paths.

```rust,edition2021
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

fn main() {
    let mut path = PathBuf::from("/etc/nginx");
    path.push("sites-available");
    path.push("default.conf");

    assert_eq!(path.parent(), Some(Path::new("/etc/nginx/sites-available")));
    assert_eq!(path.file_name(), Some(OsStr::new("default.conf")));
    assert_eq!(path.file_stem(), Some(OsStr::new("default")));
    assert_eq!(path.extension(), Some(OsStr::new("conf")));

    // Derive a backup name and a sibling path without mutating `path`.
    let backup = path.with_extension("conf.bak");
    assert_eq!(backup.file_name(), Some(OsStr::new("default.conf.bak")));

    let sibling = path.with_file_name("nginx.conf");
    assert_eq!(sibling, Path::new("/etc/nginx/sites-available/nginx.conf"));

    // Borrow a `Path` as `&str` for APIs that need one.
    let s: &str = path.to_str().expect("path is valid UTF-8");
    println!("{s}");
}
```

[`Path`]: https://doc.rust-lang.org/std/path/struct.Path.html
[`Path::to_str`]: https://doc.rust-lang.org/std/path/struct.Path.html#method.to_str
[`PathBuf`]: https://doc.rust-lang.org/std/path/struct.PathBuf.html
[`PathBuf::push`]: https://doc.rust-lang.org/std/path/struct.PathBuf.html#method.push
[`extension`]: https://doc.rust-lang.org/std/path/struct.Path.html#method.extension
[`file_name`]: https://doc.rust-lang.org/std/path/struct.Path.html#method.file_name
[`file_stem`]: https://doc.rust-lang.org/std/path/struct.Path.html#method.file_stem
[`parent`]: https://doc.rust-lang.org/std/path/struct.Path.html#method.parent
[`with_extension`]: https://doc.rust-lang.org/std/path/struct.Path.html#method.with_extension
[`with_file_name`]: https://doc.rust-lang.org/std/path/struct.Path.html#method.with_file_name
