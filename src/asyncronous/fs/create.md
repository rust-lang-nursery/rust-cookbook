## Create Files and Directories

[![tokio-badge]][tokio] [![std-badge]][std]

[`tokio::fs::File::create`] creates a new file asynchronously and overwrites the file if it exists.
[`tokio::fs::create_dir`] creates a single directory and fails if it exists. 
[`tokio::fs::create_dir_all`] creates a directory and all missing parent directories along the path.

```rust,edition2018,no_run
use std::io;
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    // create a file
    File::create("data.txt").await?;

    // create a single directory
    tokio::fs::create_dir("my_dir").await?;

    // create directory and missing parents
    tokio::fs::create_dir_all("my_dir/sub_dir/inner").await?;

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
[`tokio::fs::File::create`]: https://docs.rs/tokio/*/tokio/fs/struct.File.html#method.create
[`tokio::fs::create_dir_all`]: https://docs.rs/tokio/*/tokio/fs/fn.create_dir_all.html
[`tokio::fs::create_dir`]: https://docs.rs/tokio/*/tokio/fs/fn.create_dir.html
