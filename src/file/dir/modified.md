## File names that have been modified in the last 24 hours

[![walkdir-badge]][walkdir] [![cat-filesystem-badge]][cat-filesystem]

Gets the current working directory and returns file names modified within the last 24 hours.
[`env::current_dir`] gets the current working directory, [`WalkDir::new`] creates a new [`WalkDir`] for the current directory.
[`WalkDir::into_iter`] creates an iterator, [`Iterator::filter_map`] applies [`Result::ok`] to [`WalkDir::DirEntry`] and filters out the directories.

[`std::fs::Metadata::modified`] returns the [`SystemTime::elapsed`] time since the last modification.
[`Duration::as_secs`] converts the time to seconds and compared with 24 hours (24 * 60 * 60 seconds).
[`Iterator::for_each`] prints the file names.

```rust,edition2021
use walkdir::WalkDir;
use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    let current_dir = env::current_dir()?;
    println!("Entries modified in the last 24 hours in {:?}:", current_dir);

    for entry in WalkDir::new(current_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.metadata().unwrap().is_file()) {
        let path = entry.path();
        let metadata = entry.metadata()?;
        let modified = metadata.modified()?.elapsed()?.as_secs();
        if modified < 24 * 3600 {
            println!("{}", path.display());
        }
    }

    Ok(())
}
```

[`Duration::as_secs`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.as_secs
[`env::current_dir`]: https://doc.rust-lang.org/std/env/fn.current_dir.html
[`Iterator::filter_map`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map
[`Iterator::for_each`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.for_each
[`Result::ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.ok
[`std::fs::Metadata::modified`]: https://doc.rust-lang.org/std/fs/struct.Metadata.html#method.modified
[`SystemTime::elapsed`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.elapsed
[`WalkDir`]: https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html
[`WalkDir::DirEntry`]: https://docs.rs/walkdir/*/walkdir/struct.DirEntry.html
[`WalkDir::into_iter`]: https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html#method.into_iter
[`WalkDir::new`]: https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html#method.new
