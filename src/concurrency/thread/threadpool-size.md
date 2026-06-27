## Sum the size of every file in a directory

[![threadpool-badge]][threadpool] [![std-badge]][std] [![walkdir-badge]][walkdir] [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem]

This recipe adds up the size of every file under the book's `src` directory.
Reading a file blocks while the operating system fetches its bytes, so the work
is IO-bound: each thread spends most of its time waiting on the disk rather than
computing. A [`ThreadPool`] sized to [`std::thread::available_parallelism`] keeps
several reads in flight at once, and each worker returns its result over an
[`std::sync::mpsc`] channel.

[`WalkDir`] yields every entry under `src`. Each file is handed to [`execute`],
which reads it with [`std::fs::read`] and sends back the byte count, or the
[`io::Error`] if the read fails. The main thread sums the counts as they arrive.
Reading the contents is what keeps the workers busy with real IO;
[`std::fs::metadata`] would be cheaper when only the length matters.

```rust,edition2021
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::thread::available_parallelism;
use threadpool::ThreadPool;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let pool = ThreadPool::new(available_parallelism()?.get());
    let (tx, rx) = channel();

    let mut files = 0;
    for entry in WalkDir::new("src")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
    {
        let path: PathBuf = entry.path().to_owned();
        let tx = tx.clone();
        files += 1;
        pool.execute(move || {
            let size = fs::read(&path).map(|bytes| bytes.len());
            tx.send(size).expect("worker could not send result");
        });
    }
    drop(tx);

    let mut total = 0;
    for size in rx.iter() {
        total += size?;
    }

    println!("{files} files, {total} bytes");
    Ok(())
}
```

[`execute`]: https://docs.rs/threadpool/*/threadpool/struct.ThreadPool.html#method.execute
[`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
[`std::fs::metadata`]: https://doc.rust-lang.org/std/fs/fn.metadata.html
[`std::fs::read`]: https://doc.rust-lang.org/std/fs/fn.read.html
[`std::sync::mpsc`]: https://doc.rust-lang.org/std/sync/mpsc/index.html
[`std::thread::available_parallelism`]: https://doc.rust-lang.org/std/thread/fn.available_parallelism.html
[`ThreadPool`]: https://docs.rs/threadpool/*/threadpool/struct.ThreadPool.html
[`WalkDir`]: https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html
