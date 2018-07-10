##  Traverse directories while skipping dotfiles

[![walkdir-badge]][walkdir] [![cat-filesystem-badge]][cat-filesystem]

Uses [`filter_entry`] to descend recursively into entries passing the
`is_not_hidden` predicate thus skipping hidden files and directories.
 [`Iterator::filter`] applies to each [`WalkDir::DirEntry`] even if the parent
 is a hidden directory.

Root dir `"."` yields through [`WalkDir::depth`] usage in `is_not_hidden`
predicate.

```rust,no_run
extern crate walkdir;

use walkdir::{DirEntry, WalkDir};

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
         .file_name()
         .to_str()
         .map(|s| entry.depth() == 0 || !s.starts_with("."))
         .unwrap_or(false)
}

fn main() {
    WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .for_each(|x| println!("{}", x.path().display()));
}
```

[`filter_entry`]: https://docs.rs/walkdir/*/walkdir/struct.IntoIter.html#method.filter_entry
[`Iterator::filter`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
[`WalkDir::depth`]: https://docs.rs/walkdir/*/walkdir/struct.DirEntry.html#method.depth
[`WalkDir::DirEntry`]: https://docs.rs/walkdir/*/walkdir/struct.DirEntry.html
