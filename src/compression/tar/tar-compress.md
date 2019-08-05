## ディレクトリをtarに圧縮する

[![flate2-badge]][flate2] [![tar-badge]][tar] [![cat-compression-badge]][cat-compression]

`/var/log`ディレクトリを`archive.tar.gz`に圧縮する。

[`GzEncoder`]と[`tar::Builder`]でラップされた[`File`]を作る。  
[`Builder::append_dir_all`]を使って`var/log`ディレクトリの中身を再帰的に`backup/logs`パスの下のarchiveに追加する。[`GzEncoder`]は、`archive.tar.gz`に書き込む前にデータを透過的に圧縮します。

```rust,no_run
extern crate tar;
extern crate flate2;

use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;

fn main() -> Result<(), std::io::Error> {
    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("backup/logs", "/var/log")?;
    Ok(())
}
```

[`Builder::append_dir_all`]: https://docs.rs/tar/*/tar/struct.Builder.html#method.append_dir_all
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`GzEncoder`]: https://docs.rs/flate2/*/flate2/write/struct.GzEncoder.html
[`tar::Builder`]: https://docs.rs/tar/*/tar/struct.Builder.html
