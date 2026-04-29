# Ctrl-C

[![tokio-badge]][tokio]

Sometimes you want to run a task, but also be ready to stop everything if the user presses Ctrl-C.
[`tokio::select!`] lets you do exactly that. It runs multiple things at the same time and stops
as soon as the first one finishes, cancelling everything else.

In this example, the program tries to fetch some data. If the user presses Ctrl-C before the fetch
is done, the program shuts down cleanly instead of waiting.

Both branches start at the same time. Whichever one finishes first wins and the other is cancelled.
If fetch_data completes first, the result is printed and the loop runs again. 
If Ctrl-C comes in first, the program exits.

```rust,edition2018,no_run
use tokio::signal;
use tokio::time::{Duration, sleep};

async fn fetch_data() -> String {
    // simulate a slow network request
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

[`macros`]: https://docs.rs/crate/tokio/*/features#macros
[`signal`]: https://docs.rs/crate/tokio/*/features#signal
[`tokio::select!`]: https://docs.rs/tokio/latest/tokio/macro.select.html
