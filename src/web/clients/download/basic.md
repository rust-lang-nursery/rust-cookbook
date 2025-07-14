## Download a file to a temporary directory

[![reqwest-badge]][reqwest] [![tempfile-badge]][tempfile] [![cat-net-badge]][cat-net] [![cat-filesystem-badge]][cat-filesystem]

Creates a temporary directory with [`tempfile::Builder`] and downloads
a file over HTTP using [`reqwest::get`] asynchronously.

Creates a target [`File`] with name obtained from [`Response::url`] within
[`tempfile::TempDir::path`] and copies downloaded data to it with [`io::copy`].
The temporary directory is automatically removed on program exit.

```rust,edition2021,no_run
use anyhow::Result;
use std::io::Write;
use std::fs::File;
use tempfile::Builder;

fn main() -> Result<()> {
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response = reqwest::blocking::get(target)?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let fname = tmp_dir.path().join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname)?
    };
    let content =  response.bytes()?;
    dest.write_all(&content)?;
    Ok(())
}
```

[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`Response::url`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html#method.url
[`tempfile::Builder`]: https://docs.rs/tempfile/*/tempfile/struct.Builder.html
[`tempfile::TempDir::path`]: https://docs.rs/tempfile/*/tempfile/struct.TempDir.html#method.path
[`io::copy`]: https://doc.rust-lang.org/std/io/fn.copy.html
