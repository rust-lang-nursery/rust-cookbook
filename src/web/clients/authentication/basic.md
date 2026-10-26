## Basic Authentication

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

Uses [`reqwest::RequestBuilder::basic_auth`] to perform a basic HTTP authentication.

```rust,no_run
{{#include ../../../../crates/web/src/bin/auth_basic.rs }}
```

[`reqwest::RequestBuilder::basic_auth`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.basic_auth
