## Consume a paginated RESTful API

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

Wraps a paginated web API in a convenient Rust iterator. The iterator lazily
fetches the next page of results from the remote server as it arrives at the end of each page.

> This recipe requires the [`blocking`] and [`json`] feature flags of `reqwest` and the [`derive`] feature flag of `serde` to be enabled in `Cargo.toml`.

```rust,no_run
{{#include ../../../../crates/web/src/paginated.rs }}
```

```rust,no_run
{{#include ../../../../crates/web/src/bin/paginated.rs }}
```
[`blocking`]: https://docs.rs/crate/reqwest/*/features#blocking
[`json`]: https://docs.rs/crate/reqwest/*/features#json
[`derive`]: https://docs.rs/crate/serde/*/features#derive
