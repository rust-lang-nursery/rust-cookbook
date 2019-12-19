## Make a HTTP GET request

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

Parses the supplied URL and makes a synchronous HTTP GET request
with [`reqwest::get`]. Prints obtained [`reqwest::Response`]
status and headers. Reads HTTP response body into an allocated [`String`]
using [`read_to_string`].

```rust,edition2018,no_run
# use error_chain::error_chain;

use std::io::Read;
#
# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#         HttpRequest(reqwest::Error);
#     }
# }

fn main() -> Result<()> {
    let mut res = reqwest::get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}
```

[`read_to_string`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string
[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`reqwest::Response`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
