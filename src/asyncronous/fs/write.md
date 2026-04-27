## Write Files

[![tokio-badge]][tokio] [![std-badge]][std]

[`tokio::fs::write`] writes bytes to a file asynchronously. It creates the file if it doesn't exist,
or overwrites it if it does.

```rust,edition2018,no_run
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let content = b"Generic data from program!";
    tokio::fs::write("data.txt", content).await?;

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
[`tokio::fs::write`]: https://docs.rs/tokio/*/tokio/fs/fn.write.html
