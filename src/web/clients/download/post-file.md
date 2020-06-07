## POST a file to paste-rs

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

[`reqwest::Client`] establishes a connection to https://paste.rs
following the [`reqwest::RequestBuilder`] pattern.  Calling [`Client::post`]
with a URL establishes the destination, [`RequestBuilder::body`] sets the
content to send by reading the file, and [`RequestBuilder::send`] blocks until
the file uploads and the response returns.  [`read_to_string`] returns the
response and displays in the console.

```rust,edition2018,no_run
use error_chain::error_chain;
use std::fs::File;
use std::io::Read;

 error_chain! {
     foreign_links {
         HttpRequest(reqwest::Error);
         IoError(::std::io::Error);
     }
 }
 #[tokio::main]

async fn main() -> Result<()> {
    let paste_api = "https://paste.rs";
    let mut file = File::open("message")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let client = reqwest::Client::new();
    let res = client.post(paste_api)
        .body(contents)
        .send()
        .await?;
    let response_text = res.text().await?;
    println!("Your paste is located at: {}",response_text );
    Ok(())
}
```

[`Client::post`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.post
[`read_to_string`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string
[`RequestBuilder::body`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.body
[`RequestBuilder::send`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.send
[`reqwest::Client`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html
[`reqwest::RequestBuilder`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html
