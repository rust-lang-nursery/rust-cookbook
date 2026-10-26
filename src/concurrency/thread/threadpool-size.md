## Sum the size of every file in a directory

[![rayon-badge]][rayon] [![walkdir-badge]][walkdir] [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem]

This recipe adds up the size of every file under the book's `src` directory.
Reading a file blocks while the operating system fetches its bytes, so the work
is IO-bound: each thread spends most of its time waiting on the disk rather than
computing. A fixed-size pool keeps several reads in flight at once.

[`WalkDir`] yields every entry under `src`. A [`rayon::ThreadPool`] is built with
[`ThreadPoolBuilder::num_threads`] set to
[`std::thread::available_parallelism`]. [`ThreadPool::install`] runs the closure
inside that pool, so the [`par_iter`] below uses exactly those threads instead of
rayon's global pool. Each file is read with [`std::fs::read`], and the byte count
(or the [`io::Error`] if the read fails) is collected into a single
[`io::Result`].
Reading the contents is what keeps the workers busy with real IO;
[`std::fs::metadata`] would be cheaper when only the length matters.

```rust,edition2021
use anyhow::Result;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use rayon::ThreadPoolBuilder;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::thread::available_parallelism;
use walkdir::WalkDir;

fn main() -> Result<()> {
    let pool = ThreadPoolBuilder::new()
        .num_threads(available_parallelism()?.get())
        .build()?;

    let paths: Vec<PathBuf> = WalkDir::new("src")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| e.into_path())
        .collect();

    let sizes = pool.install(|| {
        paths
            .par_iter()
            .map(|path| fs::read(path).map(|bytes| bytes.len()))
            .collect::<io::Result<Vec<usize>>>()
    })?;

    let total: usize = sizes.iter().sum();
    println!("{} files, {} bytes", paths.len(), total);
    Ok(())
}
```

[`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
[`io::Result`]: https://doc.rust-lang.org/std/io/type.Result.html
[`par_iter`]: https://docs.rs/rayon/*/rayon/iter/trait.IntoParallelRefIterator.html#tymethod.par_iter
[`rayon::ThreadPool`]: https://docs.rs/rayon/*/rayon/struct.ThreadPool.html
[`std::fs::metadata`]: https://doc.rust-lang.org/std/fs/fn.metadata.html
[`std::fs::read`]: https://doc.rust-lang.org/std/fs/fn.read.html
[`std::thread::available_parallelism`]: https://doc.rust-lang.org/std/thread/fn.available_parallelism.html
[`ThreadPool::install`]: https://docs.rs/rayon/*/rayon/struct.ThreadPool.html#method.install
[`ThreadPoolBuilder::num_threads`]: https://docs.rs/rayon/*/rayon/struct.ThreadPoolBuilder.html#method.num_threads
[`WalkDir`]: https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html
