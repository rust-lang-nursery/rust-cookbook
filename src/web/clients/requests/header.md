## Set custom headers and URL parameters for a REST request

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![url-badge]][url] [![cat-net-badge]][cat-net]

Sets both standard and custom HTTP headers as well as URL parameters for a HTTP
GET request. Creates a custom header of type `XPoweredBy` with [`header!`] macro.

Then, builds the complex URL with [`Url::parse_with_params`]. Sets standard
headers [`header::USER_AGENT`], [`header::AUTHORIZATION`] and custom
`XPoweredBy` header using [`RequestBuilder::header`], then makes the request
with [`RequestBuilder::send`].

The request target <http://httpbin.org/headers> responds with
a JSON dict containing all request headers for easy verification.

```rust,edition2021,no_run
use anyhow::Result;
use reqwest::Url;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct HeadersEcho {
    pub headers: HashMap<String, String>,
}

fn main() -> Result<()> {
    let url = Url::parse_with_params(
        "http://httpbin.org/headers",
        &[("lang", "rust"), ("browser", "servo")],
    )?;

    let response = Client::new()
        .get(url)
        .header(USER_AGENT, "Rust-test-agent")
        .header("X-Powered-By", "Rust")
        .send()?;

    assert_eq!(
        response.url().as_str(),
        "http://httpbin.org/headers?lang=rust&browser=servo"
    );

    let out: HeadersEcho = response.json()?;
    assert_eq!(out.headers["User-Agent"], "Rust-test-agent");
    assert_eq!(out.headers["X-Powered-By"], "Rust");

    Ok(())
}

[`header::USER_AGENT`]: https://docs.rs/reqwest/*/reqwest/header/constant.USER_AGENT.html
[`RequestBuilder::HeaderName::TryFrom<&'a str>`]: https://docs.rs/reqwest/*/reqwest/header/struct.HeaderName.html#impl-TryFrom%3C%26%27a%20str%3E
[`RequestBuilder::send`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.send
[`Url::parse_with_params`]: https://docs.rs/url/*/url/struct.Url.html#method.parse_with_params
