## Unbounded Channels

[![tokio-badge]][tokio] [![std-badge]][std]

An [`unbounded channel`] has no limit on how many messages it can hold. The sender never has to wait,
it can always drop a message in, no matter how many are already sitting there.

Think of it like a digital inbox. It just keeps growing as new messages arrive. There's no cap, 
but if messages pile up faster than they're being read, your program will use more and more memory.
Sending on an unbounded channel will always succeed as long as the receiving end is still open.
If the receiver is slow, messages simply queue up and wait.

In this example, two people send messages through a channel. An inbox collects whatever comes 
through.

```rust,edition2018
use std::io;
use tokio::sync::mpsc::unbounded_channel;

struct Message {
    from: &'static str,
    text: &'static str,
}

impl Message {
    fn new(from: &'static str, text: &'static str) -> Self {
        Self { from, text }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let (message_sender, mut message_receiver) = unbounded_channel();

    let alice_message_sender = message_sender.clone();
    let person_one = tokio::task::spawn(async move {
        if let Err(err) = alice_message_sender.send(Message::new("Alice", "Meeting postponed")) {
            eprintln!("Failed to send message from Alice: {}", err);
        }
    });

    let person_two = tokio::task::spawn(async move {
        if let Err(err) = message_sender.send(Message::new("Bob", "Secret Leaked")) {
            eprintln!("Failed to send message from Bob: {}", err);
        }
    });

    let mut inbox: Vec<Message> = Vec::new();
    while let Some(new_book) = message_receiver.recv().await {
        inbox.push(new_book);
    }

    person_one.await?;
    person_two.await?;

    for msg in &inbox {
        println!("{} says: {}", msg.from, msg.text);
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
