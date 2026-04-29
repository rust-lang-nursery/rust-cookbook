## Read files

[![tokio-badge]][tokio] [![std-badge]][std]

Reading a file from disk takes time. Tokio provides non-blocking versions of file reads so your
program can keep doing other work while waiting for the data to come back.

- [`read`] loads the file into raw bytes. Useful when you need to process the data directly.
- [`read_to_string`] loads the file into plain text. Useful when you know the file contains readable
characters.

```rust,edition2018,no_run
use std::io;

fn process_data(data: &[u8]) {
    println!("Data Length: {}", data.len());
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // read to a Vec<u8>
    let content = tokio::fs::read("data.txt").await?;
    process_data(&content);

    // read to a String
    let str_content = tokio::fs::read_to_string("data.txt").await?;
    process_data(str_content.as_bytes());

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
[`read_to_string`]: https://docs.rs/tokio/*/tokio/fs/fn.read_to_string.html
[`read`]: https://docs.rs/tokio/*/tokio/fs/fn.read.html
