## Download a file to a temporary directory

[![reqwest-badge]][reqwest] [![tempfile-badge]][tempfile] [![cat-net-badge]][cat-net] [![cat-filesystem-badge]][cat-filesystem]

Creates a temporary directory with [`tempfile::Builder`] and downloads
a file over HTTP using [`reqwest::get`] asynchronously.

Creates a target [`File`] with name obtained from [`Response::url`] within
[`tempdir()`] and writes downloaded data into it with [`Writer::write_all`].
The temporary directory is automatically removed on program exit.

```rust,edition2018,no_run
use error_chain::error_chain;
use std::io::Write;
use std::fs::File;
use tempfile::Builder;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

#[tokio::main]
async fn main() -> Result<()> {
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response = reqwest::get(target).await?;

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
    let content =  response.bytes().await?;
    dest.write_all(&content)?;
    Ok(())
}
```

[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`Response::url`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html#method.url
[`tempfile::Builder`]: https://docs.rs/tempfile/*/tempfile/struct.Builder.html
[`tempdir()`]: https://docs.rs/tempfile/*/tempfile/struct.Builder.html#method.tempdir
[`Writer::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
