## Set custom headers and URL parameters for a REST request

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![url-badge]][url] [![cat-net-badge]][cat-net]

Sets both standard and custom HTTP headers as well as URL parameters for a HTTP
GET request.

Builds a complex URL with [`Url::parse_with_params`]. Sets standard
headers [`header::USER_AGENT`] and [`header::AUTHORIZATION`], plus a custom
`X-Powered-By` header using [`RequestBuilder::header`], then makes the request
with [`RequestBuilder::send`].

The request target <http://httpbin.org/headers> responds with
a JSON dict containing all request headers for easy verification.

```rust,no_run
{{#include ../../../../crates/web/src/bin/header.rs }}
```

[`header::AUTHORIZATION`]: https://docs.rs/reqwest/latest/reqwest/header/constant.AUTHORIZATION.html
[`header::USER_AGENT`]: https://docs.rs/reqwest/latest/reqwest/header/constant.USER_AGENT.html
[`RequestBuilder::header`]: https://docs.rs/reqwest/latest/reqwest/blocking/struct.RequestBuilder.html#method.header
[`RequestBuilder::send`]: https://docs.rs/reqwest/latest/reqwest/blocking/struct.RequestBuilder.html#method.send
[`Url::parse_with_params`]: https://docs.rs/url/latest/url/struct.Url.html#method.parse_with_params


