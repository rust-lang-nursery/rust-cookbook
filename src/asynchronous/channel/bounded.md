## Bounded Channels

[![tokio-badge]][tokio] [![std-badge]][std]

A [`bounded channel`] has a limit on how many messages it can hold at once. If the channel is full,
the sender has to wait until there is room before it can send another message.

Think of it like a physical mailbox. It can only hold a fixed number of letters. If it's full, 
the postman has to wait until someone clears it out before dropping in another letter.

In this example, two book stores send books through a channel with a capacity of 5. 
A shelf collects whatever comes through.

```rust,edition2018
use std::io;
use tokio::sync::mpsc::channel;

struct Book {
    title: &'static str,
}

impl Book {
    fn new(title: &'static str) -> Self {
        Self { title }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let (book_sender, mut book_receiver) = channel(5);

    // Each store gets its own copy of the sender.
    let store_one_book_sender = book_sender.clone();
    let book_store_one = tokio::task::spawn(async move {
        if let Err(err) = store_one_book_sender
            .send(Book::new("Shawshank Redemption"))
            .await
        {
            eprintln!("Failed to send book from store one: {}", err);
        }
    });

    let book_store_two = tokio::task::spawn(async move {
        if let Err(err) = book_sender.send(Book::new("Secret Recipe")).await {
            eprintln!("Failed to send book from store two: {}", err);
        }
    });

    let mut shelf: Vec<Book> = Vec::new();
    // Collect every book that arrives until both stores are done sending.
    while let Some(new_book) = book_receiver.recv().await {
        shelf.push(new_book);
    }

    book_store_one.await?;
    book_store_two.await?;

    for book in &shelf {
        println!("Title: {}", book.title);
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
