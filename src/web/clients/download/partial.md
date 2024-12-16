## Make a partial download with 

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

Uses [`reqwest::Client::head`] to get the [Content-Length] of the response.

The code then uses [`chunk`] to download the content in chunks writing
to a local file.

```rust,edition2024,no_run
use anyhow::{Error, Result};
use reqwest::header::{CONTENT_LENGTH};
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
  let url = "https://httpbin.org/range/102400?duration=2";
    
  let client = reqwest::Client::new();
  let header = client.head(url).send().await?;
  let length = header
    .headers()
    .get(CONTENT_LENGTH);
  let length = u64::from_str(
      length.expect("Content Length not provided").to_str()?
      ).map_err(Error::msg)?;
    
  let mut output_file = File::create("download.bin")?;
  let mut response = client.get(url).send().await?;
    
  println!("starting download...");
  while let Some(chunk) = response.chunk().await? {
      println!("Received chunk, writing to file");
      output_file.write_all(&chunk)?;
  }

  println!("Finished with success! {} bytes", length);
  Ok(())
}
```

[`reqwest::Client::head`]: https://docs.rs/reqwest/*/reqwest/blocking/struct.Client.html#method.head
[`chunk`]: https://docs.rs/reqwest/latest/reqwest/struct.Response.html#method.chunk
[Content-Length]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Length
