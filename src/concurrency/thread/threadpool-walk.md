## Calculate SHA256 sum of iso files concurrently

[![rayon-badge]][rayon] [![walkdir-badge]][walkdir] [![ring-badge]][ring] [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem]

This recipe calculates the SHA256 for every file with iso extension in the
current directory. [`Walkdir::new`] iterates the directory and the matching
paths are collected into a `Vec`. [`par_iter`] then spreads the work of reading
each file and computing its hash across rayon's global thread pool, which has
one thread per available core by default. The digests are gathered with
[`collect`] into a single [`Result`], so the first error stops the run.

```rust,edition2021,no_run
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

# // Verify the iso extension
# fn is_iso(entry: &Path) -> bool {
#     match entry.extension() {
#         Some(e) if e.to_string_lossy().to_lowercase() == "iso" => true,
#         _ => false,
#     }
# }

fn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<Digest, Error> {
    let mut buf_reader = BufReader::new(File::open(filepath)?);
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = buf_reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

fn main() -> Result<(), Error> {
    let paths: Vec<PathBuf> = WalkDir::new("/home/user/Downloads")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir() && is_iso(e.path()))
        .map(|e| e.into_path())
        .collect();

    let digests = paths
        .par_iter()
        .map(|path| compute_digest(path).map(|digest| (digest, path)))
        .collect::<Result<Vec<_>, Error>>()?;

    for (sha, path) in digests {
        println!("{:?} {:?}", sha, path);
    }
    Ok(())
}
```

[`collect`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.collect
[`par_iter`]: https://docs.rs/rayon/*/rayon/iter/trait.IntoParallelRefIterator.html#tymethod.par_iter
[`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
[`Walkdir::new`]: https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html#method.new
