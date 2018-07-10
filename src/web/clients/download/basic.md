## Download a file to a temporary directory

[![reqwest-badge]][reqwest] [![tempdir-badge]][tempdir] [![cat-net-badge]][cat-net] [![cat-filesystem-badge]][cat-filesystem]

Creates a temporary directory with [`TempDir::new`] and synchronously downloads
a file over HTTP using [`reqwest::get`].

Creates a target [`File`] with name obtained from [`Response::url`] within
[`TempDir::path`] and copies downloaded data into it with [`io::copy`].
The temporary directory is automatically removed on `run` function return.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate reqwest;
extern crate tempdir;

use std::io::copy;
use std::fs::File;
use tempdir::TempDir;
#
# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#         HttpRequest(reqwest::Error);
#     }
# }

fn run() -> Result<()> {
    let tmp_dir = TempDir::new("example")?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let mut response = reqwest::get(target)?;

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
    copy(&mut response, &mut dest)?;
    Ok(())
}
#
# quick_main!(run);
```

[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`io::copy`]: https://doc.rust-lang.org/std/io/fn.copy.html
[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`Response::url`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html#method.url
[`TempDir::new`]: https://docs.rs/tempdir/*/tempdir/struct.TempDir.html#method.new
[`TempDir::path`]: https://docs.rs/tempdir/*/tempdir/struct.TempDir.html#method.path
