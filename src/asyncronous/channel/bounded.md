## Bounded Channels

[![tokio-badge]][tokio] [![std-badge]][std]

This examples shows the use of a [`bounded channel`]. A bounded channel will buffer up to the provided 
number of messages. Once the buffer is full, attempts to send new messages will wait until a
message is received from the channel. The provided buffer capacity must be at least 1.
This can be used to apply backpressure and prevent a fast sender from overwhelming a slow receiver.

```rust,edition2018
use std::io;
use tokio::sync::mpsc::channel;

#[tokio::main]
async fn main() -> io::Result<()> {
    let (msg_tx, mut msg_rx) = channel(5);

    tokio::task::spawn(async move {
        for x in 0..5 {
            if msg_tx.send(x).await.is_err() {
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

[`bounded channel`]: https://docs.rs/tokio/*/tokio/sync/mpsc/fn.channel.html
[`macros`]: https://docs.rs/crate/tokio/*/features#macros
[`sync`]: https://docs.rs/crate/tokio/*/features#sync
