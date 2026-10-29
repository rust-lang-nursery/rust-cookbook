## Parse the MIME type of a HTTP response

[![reqwest-badge]][reqwest] [![mime-badge]][mime] [![tokio-badge]][tokio] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

When receiving a HTTP reponse from `reqwest` the [MIME type] or media type may be
found in the [Content-Type] header. [`reqwest::header::HeaderMap::get`] retrieves
the header as a [`reqwest::header::HeaderValue`], which can be converted to a
string. The `mime` crate can then parse that, yielding a [`mime::Mime`] value.

The [`mime`] crate also defines some commonly used MIME types.

Note that the [`reqwest::header`] module is exported from the [`http`] crate.

> This recipe requires the [`macros`] and [`rt-multi-thread`] feature flags of `tokio` to be enabled in `Cargo.toml`.

```rust,no_run
{{#include ../../../crates/web/src/bin/mime_request.rs }}
```

[`http`]: https://docs.rs/http/*/http/
[`mime::Mime`]: https://docs.rs/mime/*/mime/struct.Mime.html
[`reqwest::header::HeaderMap::get`]: https://docs.rs/reqwest/*/reqwest/header/struct.HeaderMap.html#method.get
[`reqwest::header::HeaderValue`]: https://docs.rs/reqwest/*/reqwest/header/struct.HeaderValue.html
[`reqwest::header`]: https://docs.rs/reqwest/*/reqwest/header/index.html

[Content-Type]: https://developer.mozilla.org/docs/Web/HTTP/Headers/Content-Type
[MIME type]: https://developer.mozilla.org/docs/Web/HTTP/Basics_of_HTTP/MIME_types
[`mime`]: https://docs.rs/mime/*/mime/
[`macros`]: https://docs.rs/crate/tokio/*/features#macros
[`rt-multi-thread`]: https://docs.rs/crate/tokio/*/features#rt-multi-thread
<!-- needs-no-features: reqwest -->
