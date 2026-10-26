## Check if an API resource exists

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

Query the GitHub Users Endpoint using a HEAD request ([`Client::head`]) and then
inspect the response code to determine success. This is a quick way to query a
rest resource without needing to receive a body. [`reqwest::Client`] configured
with [`ClientBuilder::timeout`] ensures a request will not last longer than a
timeout. The client also sets a [`ClientBuilder::user_agent`], which the GitHub API
requires on every request.

Due to both [`ClientBuilder::build`] and [`RequestBuilder::send`] returning [`reqwest::Error`]
types, the shortcut [`reqwest::Result`] is used for the main function return type. 

```rust,no_run
{{#include ../../../../crates/web/src/bin/rest_head.rs }}
```

[`ClientBuilder::build`]: https://docs.rs/reqwest/*/reqwest/struct.ClientBuilder.html#method.build
[`Client::head`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.head
[`ClientBuilder::timeout`]: https://docs.rs/reqwest/*/reqwest/struct.ClientBuilder.html#method.timeout
[`RequestBuilder::send`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.send
[`reqwest::Client`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html
[`reqwest::Error`]: https://docs.rs/reqwest/*/reqwest/struct.Error.html
[`reqwest::Result`]:https://docs.rs/reqwest/*/reqwest/type.Result.html
[`ClientBuilder::user_agent`]: https://docs.rs/reqwest/*/reqwest/struct.ClientBuilder.html#method.user_agent
