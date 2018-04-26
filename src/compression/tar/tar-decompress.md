[ex-tar-decompress]: #ex-tar-decompress
<a name="ex-tar-decompress"></a>
## Decompress a tarball

[![flate2-badge]][flate2] [![tar-badge]][tar] [![cat-compression-badge]][cat-compression]

Decompress ([`GzDecoder`]) and
extract ([`Archive::unpack`]) all files from a compressed tarball
named `archive.tar.gz` located in the current working directory.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate flate2;
extern crate tar;

use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;
#
# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#     }
# }

fn run() -> Result<()> {
    let path = "archive.tar.gz";

    // Open a compressed tarball
    let tar_gz = File::open(path)?;
    // Decompress it
    let tar = GzDecoder::new(tar_gz);
    // Load the archive from the tarball
    let mut archive = Archive::new(tar);
    // Unpack the archive inside curent working directory
    archive.unpack(".")?;

    Ok(())
}
#
# quick_main!(run);
```

[`Archive::unpack`]: https://docs.rs/tar/*/tar/struct.Archive.html#method.unpack
[`GzDecoder`]: https://docs.rs/flate2/*/flate2/read/struct.GzDecoder.html
