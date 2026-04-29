# Macro
[![tokio-badge]][tokio]

The easiest way to start the Tokio runtime is with the [`tokio::main`] macro. Put it above your
main function and it handles everything for you, starting the runtime, running your code, and 
shutting it down when done. It also lets `main` be non-blocking, so you can use `.await` directly
inside it.

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
