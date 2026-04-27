## Read files

[![tokio-badge]][tokio] [![std-badge]][std]

[`tokio::fs`] provides async versions of the standard file operations. [`read`] loads the file into a 
`Vec<u8>` and [`read_to_string`] loads it into a [`String`]. Both of these functions do not block the 
current thread while waiting for the disk.

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

[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
[`fs`]: https://docs.rs/crate/tokio/*/features#fs
[`macros`]: https://docs.rs/crate/tokio/*/features#macros
[`read_to_string`]: https://docs.rs/tokio/*/tokio/fs/fn.read_to_string.html
[`read`]: https://docs.rs/tokio/*/tokio/fs/fn.read.html
[`tokio::fs`]: https://docs.rs/tokio/*/tokio/fs/index.html
