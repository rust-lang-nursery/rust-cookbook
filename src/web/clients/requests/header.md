## Set custom headers and URL parameters for a REST request

[![reqwest-badge]][reqwest] [![hyper-badge]][hyper] [![url-badge]][url] [![cat-net-badge]][cat-net]

Sets both standard and custom HTTP headers as well as URL parameters
for HTTP GET request. Firstly creates a custom header of type `XPoweredBy`
with [`hyper::header!`] macro. Secondly calls [`Url::parse_with_params`]
in order to build a complex URL with specified key value pairs.
Lastly sets standard headers [`header::UserAgent`] and [`header::Authorization`]
as well as custom one `XPoweredBy` with [`RequestBuilder::header`] prior to making
the request with [`RequestBuilder::send`].

The code is run against <http://httpbin.org/headers> service which responds with
a JSON dict containing all request headers for easy verification.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate url;
extern crate reqwest;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use url::Url;
use reqwest::Client;
use reqwest::header::{UserAgent, Authorization, Bearer};

// Custom header type
header! { (XPoweredBy, "X-Powered-By") => [String] }

// Helper for verification
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

fn run() -> Result<()> {
    // Make request to webservice that will respond with JSON dict containing
    // the headders set on HTTP GET request.
    let url = Url::parse_with_params("http://httpbin.org/headers",
                                     &[("lang", "rust"), ("browser", "servo")])?;

    let mut response = Client::new()
        .get(url)
        .header(UserAgent::new("Rust-test"))
        .header(Authorization(Bearer { token: "DEadBEEfc001cAFeEDEcafBAd".to_owned() }))
        .header(XPoweredBy("Guybrush Threepwood".to_owned()))
        .send()?;

    // JSON response should match the headers set on request
    let out: HeadersEcho = response.json()?;
    assert_eq!(out.headers["Authorization"],
               "Bearer DEadBEEfc001cAFeEDEcafBAd");
    assert_eq!(out.headers["User-Agent"], "Rust-test");
    assert_eq!(out.headers["X-Powered-By"], "Guybrush Threepwood");
    // Response contains full URL used to make the request
    assert_eq!(response.url().as_str(),
               "http://httpbin.org/headers?lang=rust&browser=servo");

    println!("{:?}", out);
    Ok(())
}
#
# quick_main!(run);
```

[`header::Authorization`]: https://docs.rs/hyper/*/hyper/header/struct.Authorization.html
[`header::UserAgent`]: https://docs.rs/hyper/*/hyper/header/struct.UserAgent.html
[`hyper::header!`]: https://docs.rs/hyper/*/hyper/macro.header.html
[`RequestBuilder::header`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.header
[`RequestBuilder::send`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.send
[`Url::parse_with_params`]: https://docs.rs/url/*/url/struct.Url.html#method.parse_with_params
