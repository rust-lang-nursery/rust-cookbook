# Timeouts

[![tokio-badge]][tokio] [![std-badge]][std]

Sometimes a task takes too long and you don't want to wait forever. A [`timeout`] puts a time limit
on a task. If it finishes in time you get the result, if it doesn't, it is cancelled and you get
an error back.

In this example, a network request is given 5 milliseconds to complete. If it finishes in time, 
the result is printed. If it runs over, the program reports a timeout.

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
[`timeout`]: https://docs.rs/tokio/*/tokio/time/fn.timeout.html
