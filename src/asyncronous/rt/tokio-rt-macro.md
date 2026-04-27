# Tokio Runtime
[![tokio-badge]][tokio]

This example uses [`tokio::main`] macro to set up an async runtime and mark the entry point of 
the program. It allows main to be async, so you can use .await directly inside it.

```rust,edition2018
async fn fetch_network_request() -> u32 {
    89
}

#[tokio::main]
async fn main() {
    let result = fetch_network_request().await;
    assert_eq!(result, 89);
}
```

> Add `tokio` to `Cargo.toml` with the [`macros`] and [`rt-multi-thread`] features enabled.
> ```toml
> [dependencies]
> tokio = { version = "*", features = ["macros", "rt-multi-thread"] }
> ```

[`macros`]: https://docs.rs/crate/tokio/*/features#macros
[`rt-multi-thread`]: https://docs.rs/crate/tokio/*/features#rt-multi-thread
[`tokio::main`]: https://docs.rs/tokio/*/tokio/attr.main.html
