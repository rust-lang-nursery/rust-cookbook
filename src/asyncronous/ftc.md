# Select First to Complete

[![tokio-badge]][tokio]

This example shows how to handle Ctrl-C signal. [`tokio::select`] runs multiple async branches at
the same time and returns as soon as the first one finishes and cancels the rest.
This would be useful when racing a task against a timeout or a shutdown signal. 

```rust,edition2018,no_run
use tokio::signal;
use tokio::time::{Duration, sleep};

async fn fetch_data() -> String {
    sleep(Duration::from_secs(5)).await;
    "data".to_string()
}

#[tokio::main]
async fn main() {
    loop {
        tokio::select! {
            result = fetch_data() => {
                println!("Got: {}", result);
            }
            _ = signal::ctrl_c() => {
                println!("Shutting down...");
                return;
            }
        }
    }
}
```

> Add `tokio` to `Cargo.toml` with the [`macros`] and [`signal`] features enabled.
> ```toml
> [dependencies]
> tokio = { version = "*", features = ["macros", "signal"] }
> ```

{{#include ../links.md}}

[`macros`]: https://docs.rs/crate/tokio/*/features#macros
[`signal`]: https://docs.rs/crate/tokio/*/features#signal
[`tokio::select`]: https://docs.rs/tokio/latest/tokio/macro.select.html
