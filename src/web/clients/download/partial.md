## Make a partial download with HTTP range headers

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

Uses [`reqwest::blocking::Client::head`] to get the [Content-Length] of the
response.

The code then uses [`reqwest::blocking::Client::get`] to download the content
in chunks of 10240 bytes, while printing progress messages. This approach is
useful to control memory usage for large files and allows for resumable
downloads.

The Range header is defined in [RFC7233][HTTP Range RFC7233].

> This recipe requires the [`blocking`] feature flag of `reqwest` to be enabled in `Cargo.toml`.

```rust,no_run
{{#include ../../../../crates/web/src/bin/download_partial.rs }}
```

[`reqwest::blocking::Client::get`]: https://docs.rs/reqwest/*/reqwest/blocking/struct.Client.html#method.get
[`reqwest::blocking::Client::head`]: https://docs.rs/reqwest/*/reqwest/blocking/struct.Client.html#method.head
[Content-Length]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Length
[Range]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Range

[HTTP Range RFC7233]: https://tools.ietf.org/html/rfc7233#section-3.1
[`blocking`]: https://docs.rs/crate/reqwest/*/features#blocking
