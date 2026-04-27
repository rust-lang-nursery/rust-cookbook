## AsyncRead and AsyncWrite

[![tokio-badge]][tokio] [![std-badge]][std]

This example shows the use of [`AsyncRead`] and [`AsyncWrite`] which are the core traits for async 
I/O in Tokio. Their extensions [`AsyncReadExt`] and [`AsyncWriteExt`] add convenient methods like
[`read`], [`write`], [`read_to_end`], and [`copy`] directly on any async stream or file.

```rust,edition2018,no_run
use std::io;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // AsyncWrite: write bytes to a file
    let mut file = File::create("data.txt").await?;
    file.write_all(b"Generated Data!").await?;

    // AsyncRead: read bytes from a file
    let mut file = File::open("data.txt").await?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).await?;
    println!("Data Length: {}", contents.len());

    // stream from reader to writer
    let mut reader = File::open("data.txt").await?;
    let mut writer = File::create("copy.txt").await?;
    tokio::io::copy(&mut reader, &mut writer).await?;

    Ok(())
}
```

> Types like [`File`], [`TcpStream`], and [`BufReader`] implement the traits [`AsyncRead`] and 
[`AsyncWrite`].

> Add `tokio` to `Cargo.toml` with the [`macros`], [`io-util`] and [`fs`] features enabled.
> ```toml
> [dependencies]
> tokio = { version = "*", features = ["macros", "fs", "io-util"] }
> ```

[`AsyncReadExt`]: https://docs.rs/tokio/*/tokio/io/trait.AsyncReadExt.html
[`AsyncRead`]: https://docs.rs/tokio/*/tokio/io/trait.AsyncRead.html
[`AsyncWriteExt`]: https://docs.rs/tokio/*/tokio/io/trait.AsyncWriteExt.html
[`AsyncWrite`]: https://docs.rs/tokio/*/tokio/io/trait.AsyncWrite.html
[`BufReader`]: https://docs.rs/tokio/*/tokio/io/struct.BufReader.html
[`File`]: https://docs.rs/tokio/*/tokio/fs/struct.File.html
[`TcpStream`]: https://docs.rs/tokio/*/tokio/net/struct.TcpStream.html
[`copy`]: https://docs.rs/tokio/*/tokio/io/fn.copy.html
[`fs`]: https://docs.rs/crate/tokio/*/features#fs
[`io-util`]: https://docs.rs/crate/tokio/*/features#io-util
[`macros`]: https://docs.rs/crate/tokio/*/features#macros
[`read_to_end`]: https://docs.rs/tokio/*/tokio/io/trait.AsyncReadExt.html#method.read_to_end
[`read`]: https://docs.rs/tokio/*/tokio/io/trait.AsyncReadExt.html#method.read
[`write`]: https://docs.rs/tokio/*/tokio/io/trait.AsyncWriteExt.html#method.write
