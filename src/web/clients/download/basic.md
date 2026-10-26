## Download a file to a temporary directory

[![reqwest-badge]][reqwest] [![tempfile-badge]][tempfile] [![cat-net-badge]][cat-net] [![cat-filesystem-badge]][cat-filesystem]

Creates a temporary directory with [`tempfile::Builder`] and downloads
a file over HTTP using [`reqwest::get`] asynchronously.

Creates a target [`File`] with name obtained from [`Response::url`] within
[`tempfile::TempDir::path`] and copies downloaded data to it with [`io::copy`].
The temporary directory is automatically removed on program exit.

```rust,no_run
{{#include ../../../../crates/web/src/bin/download_basic.rs }}
```

[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`Response::url`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html#method.url
[`tempfile::Builder`]: https://docs.rs/tempfile/*/tempfile/struct.Builder.html
[`tempfile::TempDir::path`]: https://docs.rs/tempfile/*/tempfile/struct.TempDir.html#method.path
[`io::copy`]: https://doc.rust-lang.org/std/io/fn.copy.html
