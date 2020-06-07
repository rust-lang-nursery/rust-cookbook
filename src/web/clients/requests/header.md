## Set custom headers and URL parameters for a REST request

[![reqwest-badge]][reqwest] [![hyper-badge]][hyper] [![url-badge]][url] [![cat-net-badge]][cat-net]

Sets both standard and custom HTTP headers as well as URL parameters
for a HTTP GET request. Creates a custom header of type `XPoweredBy`
with [`hyper::header!`] macro.

Builds complex URL with [`Url::parse_with_params`].  Sets standard headers
[`header::UserAgent`], [`header::Authorization`], and custom `XPoweredBy`
with [`RequestBuilder::header`] then makes the request with
[`RequestBuilder::send`].

The request targets <http://httpbin.org/headers> service which responds with
a JSON dict containing all request headers for easy verification.

```rust,edition2018,no_run
# use error_chain::error_chain;
use serde::Deserialize;

use std::collections::HashMap;
use url::Url;
use reqwest::Client;
use reqwest::header::{UserAgent, Authorization, Bearer};

header! { (XPoweredBy, "X-Powered-By") => [String] }

#[derive(Deserialize, Debug)]
pub struct HeadersEcho {
    pub headers: HashMap<String, String>,
}
#
# error_chain! {
#     foreign_links {
#         Reqwest(reqwest::Error);
#         UrlParse(url::ParseError);
#     }
# }

fn main() -> Result<()> {
    let url = Url::parse_with_params("http://httpbin.org/headers",
                                     &[("lang", "rust"), ("browser", "servo")])?;

    let mut response = Client::new()
        .get(url)
        .header(UserAgent::new("Rust-test"))
        .header(Authorization(Bearer { token: "DEadBEEfc001cAFeEDEcafBAd".to_owned() }))
        .header(XPoweredBy("Guybrush Threepwood".to_owned()))
        .send()?;

    let out: HeadersEcho = response.json()?;
    assert_eq!(out.headers["Authorization"],
               "Bearer DEadBEEfc001cAFeEDEcafBAd");
    assert_eq!(out.headers["User-Agent"], "Rust-test");
    assert_eq!(out.headers["X-Powered-By"], "Guybrush Threepwood");
    assert_eq!(response.url().as_str(),
               "http://httpbin.org/headers?lang=rust&browser=servo");

    println!("{:?}", out);
    Ok(())
}
```

[`header::Authorization`]: https://doc.servo.org/hyper/header/struct.Authorization.html
[`header::UserAgent`]: https://doc.servo.org/hyper/header/struct.UserAgent.html
[`hyper::header!`]: https://doc.servo.org/hyper/macro.header.html
[`RequestBuilder::header`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.header
[`RequestBuilder::send`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.send
[`Url::parse_with_params`]: https://docs.rs/url/*/url/struct.Url.html#method.parse_with_params
