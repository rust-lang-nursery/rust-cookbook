## Unbounded Channels

[![tokio-badge]][tokio] [![std-badge]][std]

A send on [`unbounded channel`] will always succeed as long as the receive half has not been closed. 
If the receiver falls behind, messages will be arbitrarily buffered.

```rust,edition2018
use std::io;
use tokio::sync::mpsc::unbounded_channel;

#[tokio::main]
async fn main() -> io::Result<()> {
    let (msg_tx, mut msg_rx) = unbounded_channel();

    tokio::task::spawn(async move {
        for x in 0..5 {
            if msg_tx.send(x).is_err() { // Not async
                break;
            }
        }
    });

    while let Some(msg) = msg_rx.recv().await {
        // do some work
        println!("Received: {msg}");
    }

    Ok(())
}
```

> Add `tokio` to `Cargo.toml` with the [`macros`] and [`sync`] features enabled.
> ```toml
> [dependencies]
> tokio = { version = "*", features = ["macros", "sync"] }
> ```

[`macros`]: https://docs.rs/crate/tokio/*/features#macros
[`sync`]: https://docs.rs/crate/tokio/*/features#sync
[`unbounded channel`]: https://docs.rs/tokio/*/tokio/sync/mpsc/fn.unbounded_channel.html
