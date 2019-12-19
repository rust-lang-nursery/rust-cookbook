##  Recursively find all files with given predicate

[![walkdir-badge]][walkdir] [![cat-filesystem-badge]][cat-filesystem]

Find JSON files modified within the last day in the current directory.
Using [`follow_links`] ensures symbolic links are followed like they were
normal directories and files.

```rust,edition2018,no_run
# use error_chain::error_chain;

use walkdir::WalkDir;
#
# error_chain! {
#     foreign_links {
#         WalkDir(walkdir::Error);
#         Io(std::io::Error);
#         SystemTime(std::time::SystemTimeError);
#     }
# }

fn main() -> Result<()> {
    for entry in WalkDir::new(".")
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();
        let sec = entry.metadata()?.modified()?;

        if f_name.ends_with(".json") && sec.elapsed()?.as_secs() < 86400 {
            println!("{}", f_name);
        }
    }

    Ok(())
}
```

[`follow_links`]: https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html#method.follow_links
