# Timeouts

[![tokio-badge]][tokio] [![std-badge]][std]

This example shows the use of timeouts in async operations. [`tokio::time::timeout`] wraps a
future and sets a maximum time it can run. If the future finishes in time you get the result,
otherwise it is cancelled and an error is returned.

```rust,edition2018
use std::time::Duration;
use tokio::time::timeout;

async fn fetch_network_request() -> u32 {
    89
}

#[tokio::main]
async fn main() {
    match timeout(Duration::from_millis(5), fetch_network_request()).await {
        Ok(x) => println!("Received {x}"),
        Err(_) => eprintln!("Timed Out!"),
    }
}
```

> Add `tokio` to `Cargo.toml` with the [`macros`] and [`time`] features enabled.
> ```toml
> [dependencies]
> tokio = { version = "*", features = ["macros", "time"] }
> ```

{{#include ../links.md}}

[`macros`]: https://docs.rs/crate/tokio/*/features#macros
[`time`]: https://docs.rs/crate/tokio/*/features#time
[`tokio::time::timeout`]: https://docs.rs/tokio/*/tokio/time/fn.timeout.html
