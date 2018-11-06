## POST a file to paste-rs

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

[`reqwest::Client`] establishes a connection to https://paste.rs
following the [`reqwest::RequestBuilder`] pattern.  Calling [`Client::post`]
with a URL establishes the destination, [`RequestBuilder::body`] sets the
content to send by reading the file, and [`RequestBuilder::send`] blocks until
the file uploads and the response returns.  [`read_to_string`] returns the
response and displays in the console.

```rust,no_run
extern crate reqwest;

# #[macro_use]
# extern crate error_chain;
#
use std::fs::File;
use std::io::Read;
use reqwest::Client;
#
# error_chain! {
#     foreign_links {
#         HttpRequest(reqwest::Error);
#         IoError(::std::io::Error);
#     }
# }

fn run() -> Result<()> {
    let paste_api = "https://paste.rs";
    let file = File::open("message")?;

    let mut response = Client::new().post(paste_api).body(file).send()?;
    let mut response_body = String::new();
    response.read_to_string(&mut response_body)?;
    println!("Your paste is located at: {}", response_body);
    Ok(())
}
#
# quick_main!(run);
```

[`Client::post`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.post
[`read_to_string`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string
[`RequestBuilder::body`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.body
[`RequestBuilder::send`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.send
[`reqwest::Client`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html
[`reqwest::RequestBuilder`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html
