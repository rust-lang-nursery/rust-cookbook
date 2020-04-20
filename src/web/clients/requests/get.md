## Make a HTTP GET request

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

Parses the supplied URL and makes a synchronous HTTP GET request
with [`reqwest::get`]. Prints obtained [`reqwest::Response`]
status and headers. Reads HTTP response body into an allocated [`String`]
using [`read_to_string`].

```rust,no_run
extern crate reqwest;
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::get("http://httpbin.org/get").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}
```

[`read_to_string`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string
[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`reqwest::Response`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
