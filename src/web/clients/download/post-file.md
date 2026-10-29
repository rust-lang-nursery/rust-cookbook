## POST a file to paste-rs

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

[`reqwest::Client`] establishes a connection to https://paste.rs following the
[`reqwest::RequestBuilder`] pattern. Calling [`Client::post`] with a URL
establishes the destination, [`RequestBuilder::body`] sets the content to send
by reading the file, and [`RequestBuilder::send`] blocks until the file uploads
and the response returns. [`read_to_string`] returns the message from the server
response and displays in the console.

> This recipe requires the [`blocking`] feature flag of `reqwest` to be enabled in `Cargo.toml`.

```rust,no_run
{{#include ../../../../crates/web/src/bin/post_file.rs }}
```

[`Client::post`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.post
[`read_to_string`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string
[`RequestBuilder::body`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.body
[`RequestBuilder::send`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.send
[`reqwest::Client`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html
[`reqwest::RequestBuilder`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html
[`blocking`]: https://docs.rs/crate/reqwest/*/features#blocking
