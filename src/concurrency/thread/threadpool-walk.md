[ex-threadpool-walk]: #ex-threadpool-walk
<a name="ex-threadpool-walk"></a>

## Calculate SHA1 sum of iso files concurrently

[![threadpool-badge]][threadpool] [![num_cpus-badge]][num_cpus] [![walkdir-badge]][walkdir] [![ring-badge]][ring] [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem]

This example calculates the SHA1 for every file present in the current directory. A threadpool is created using the number of cpus present in the system with [`num_cpus::get`]. Then every returned by [`Walkdir::new`] is passed into this pool to perform the operations of reading and computing SHA1. At the end the program waits for all jobs to finish. To get better results, compile this program in release mode.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate walkdir;
extern crate ring;
extern crate num_cpus;
extern crate threadpool;

# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#     }
# }
#
use walkdir::WalkDir;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use ring::digest::{Context, Digest, SHA1};

# // Verify the iso extension
# fn is_iso(entry: &Path) -> bool {
#     match entry.extension() {
#         Some(e) if e.to_string_lossy().to_lowercase() == "iso" => true,
#         _ => false,
#     }
# }
#
fn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<(Digest, P)> {
    let mut buf_reader = BufReader::new(File::open(&filepath)?);
    let mut context = Context::new(&SHA1);
    let mut buffer = [0; 1024];

    loop {
        let count = buf_reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok((context.finish(), filepath))
}

fn run() -> Result<()> {
    let pool = ThreadPool::new(num_cpus::get());

    let (tx, rx) = channel();

    // Look in the current directory.
    for entry in WalkDir::new("/home/user/Downloads")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir() && is_iso(e.path())) {
            let path = entry.path().to_owned();
            let tx = tx.clone();
            pool.execute(move || {
                let digest = compute_digest(path);
                tx.send(digest).expect("Could not send data!");
            });
        }

    drop(tx);
    for t in rx.iter() {
        let (sha, path) = t?;
        println!("{:?} {:?}", sha, path);
    }
    Ok(())
}
#
# quick_main!(run);
```

[`num_cpus::get`]: https://docs.rs/num_cpus/*/num_cpus/fn.get.html
[`Walkdir::new`]: https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html#method.new