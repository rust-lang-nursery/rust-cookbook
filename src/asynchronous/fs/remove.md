## Remove Files and Directories

[![tokio-badge]][tokio] [![std-badge]][std]

Deleting from disk takes time. Tokio provides non-blocking versions of these operations so your
program does not have to stop and wait while the work is done.

- [`remove_file`] deletes a single file.
- [`remove_dir`] deletes a single directory. Only works if the directory is empty.
- [`remove_dir_all`] deletes a directory and everything inside it.

```rust,edition2018,no_run
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // remove a file
    tokio::fs::remove_file("data.txt").await?;

    // remove an empty directory
    tokio::fs::remove_dir("my_dir").await?;

    // remove a directory and all its contents
    tokio::fs::remove_dir_all("my_dir").await?;

    Ok(())
}
```

> Add `tokio` to `Cargo.toml` with the [`macros`] and [`fs`] features enabled.
> ```toml
> [dependencies]
> tokio = { version = "*", features = ["macros", "fs"] }
> ```

[`fs`]: https://docs.rs/crate/tokio/*/features#fs
[`macros`]: https://docs.rs/crate/tokio/*/features#macros
[`remove_dir_all`]: https://docs.rs/tokio/*/tokio/fs/fn.remove_dir_all.html
[`remove_dir`]: https://docs.rs/tokio/*/tokio/fs/fn.remove_dir.html
[`remove_file`]: https://docs.rs/tokio/*/tokio/fs/fn.remove_file.html
