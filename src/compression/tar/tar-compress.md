[ex-tar-compress]: #ex-tar-compress
<a name="ex-tar-compress"></a>
## Compress a directory into tarball

[![flate2-badge]][flate2] [![tar-badge]][tar] [![cat-compression-badge]][cat-compression]

Compresses `/var/log` directory into `archive.tar.gz`.

Creates a [`File`] wrapped in [`GzEncoder`]
and [`tar::Builder`]. </br>Adds contents of `/var/log` directory recursively into the archive
under `backup/logs`path with [`Builder::append_dir_all`].
[`GzEncoder`] is responsible for transparently compressing the
data prior to writing it into `archive.tar.gz`.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate tar;
extern crate flate2;
#
# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#     }
# }

use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;

fn run() -> Result<()> {
    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("backup/logs", "/var/log")?;
    Ok(())
}
#
# quick_main!(run);
```

[`Builder::append_dir_all`]: https://docs.rs/tar/*/tar/struct.Builder.html#method.append_dir_all
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`GzEncoder`]: https://docs.rs/flate2/*/flate2/write/struct.GzEncoder.html
[`tar::Builder`]: https://docs.rs/tar/*/tar/struct.Builder.html
