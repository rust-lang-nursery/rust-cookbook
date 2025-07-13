## Compress a directory into tarball

[![flate2-badge]][flate2] [![tar-badge]][tar] [![cat-compression-badge]][cat-compression]

Compress `/var/log` directory into `archive.tar.gz`.

Creates a [`File`] wrapped in [`GzEncoder`]
and [`tar::Builder`]. </br>Adds contents of `/var/log` directory recursively into the archive
under `backup/logs` path with [`Builder::append_dir_all`].
[`GzEncoder`] is responsible for transparently compressing the
data prior to writing it into `archive.tar.gz`.

```rust,edition2018,no_run
use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;

fn main() -> Result<(), std::io::Error> {
    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("backup/logs", "/var/log")?;
    tar.finish()?;
    Ok(())
}
```

To add the contents without renaming them, an empty string can be used as the first argument of [`Builder::append_dir_all`]:

```rust,edition2018,no_run
use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;

fn main() -> Result<(), std::io::Error> {
    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("", "/var/log")?;
    tar.finish()?;
    Ok(())
}
```

The default behavior of [`tar::Builder`] differs from the GNU `tar` utility's defaults [tar(1)],
notably [`tar::Builder::follow_symlinks(true)`] is the equivalent of `tar --dereference`.

[tar(1)]: https://man7.org/linux/man-pages/man1/tar.1.html
[`Builder::append_dir_all`]: https://docs.rs/tar/*/tar/struct.Builder.html#method.append_dir_all
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`GzEncoder`]: https://docs.rs/flate2/*/flate2/write/struct.GzEncoder.html
[`tar::Builder`]: https://docs.rs/tar/*/tar/struct.Builder.html
[`tar::Builder::follow_symlinks(true)`]: https://docs.rs/tar/latest/tar/struct.Builder.html#method.follow_symlinks
