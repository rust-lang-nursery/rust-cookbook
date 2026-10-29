## AsyncRead and AsyncWrite

[![tokio-badge]][tokio] [![cat-asynchronous-badge]][cat-asynchronous]

[`AsyncRead`] and [`AsyncWrite`] are the building blocks for all non-blocking I/O in Tokio. 
Any type that can be read from or written to non-blocking i.e [`file`], a [`network stream`], or 
a [`buffer`], implements one or both of these.

Their extensions [`AsyncReadExt`] and [`AsyncWriteExt`] add convenient methods on top:

- [`write_all`] writes bytes to a writer.
- [`read_to_end`] reads everything from a reader into a buffer.
- [`copy`] streams data from a reader directly into a writer.

> This recipe requires the [`macros`], [`fs`] and [`io-util`] feature flags of `tokio` to be enabled in `Cargo.toml`.

```rust,edition2024,no_run
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

[`AsyncReadExt`]: https://docs.rs/tokio/*/tokio/io/trait.AsyncReadExt.html
[`AsyncRead`]: https://docs.rs/tokio/*/tokio/io/trait.AsyncRead.html
[`AsyncWriteExt`]: https://docs.rs/tokio/*/tokio/io/trait.AsyncWriteExt.html
[`AsyncWrite`]: https://docs.rs/tokio/*/tokio/io/trait.AsyncWrite.html
[`buffer`]: https://docs.rs/tokio/*/tokio/io/struct.BufReader.html
[`file`]: https://docs.rs/tokio/*/tokio/fs/struct.File.html
[`network stream`]: https://docs.rs/tokio/*/tokio/net/struct.TcpStream.html
[`copy`]: https://docs.rs/tokio/*/tokio/io/fn.copy.html
[`fs`]: https://docs.rs/crate/tokio/*/features#fs
[`io-util`]: https://docs.rs/crate/tokio/*/features#io-util
[`macros`]: https://docs.rs/crate/tokio/*/features#macros
[`read_to_end`]: https://docs.rs/tokio/*/tokio/io/trait.AsyncReadExt.html#method.read_to_end
[`write_all`]: https://docs.rs/tokio/*/tokio/io/trait.AsyncWriteExt.html#method.write_all
