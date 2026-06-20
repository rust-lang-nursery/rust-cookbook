## Communicate between threads with `mpsc` channels

[![std-badge]][std] [![cat-concurrency-badge]][cat-concurrency]

A [`mpsc`] (multiple producer, single consumer) channel moves values from one or
more producer threads to a single consumer. [`channel`] returns a [`Sender`] and
a [`Receiver`]; producers push work with [`Sender::send`]. The "multiple
producer" half comes from cloning the [`Sender`] — every clone feeds the same
[`Receiver`].

The idiomatic way to consume a channel is to **iterate the [`Receiver`]**: the
loop blocks for each value and ends cleanly once every [`Sender`] (including
clones) has been dropped. That drop is the channel's natural shutdown signal, so
the original [`Sender`] is dropped explicitly after the producers are spawned.

```rust,edition2021
use anyhow::{anyhow, Result};
use std::sync::mpsc;
use std::thread;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    // One Sender clone per producer thread; all feed the single Receiver.
    let producers: Vec<_> = (0..3)
        .map(|id| {
            let tx = tx.clone();
            thread::spawn(move || -> Result<()> {
                for item in 0..3 {
                    tx.send((id, item)).map_err(|e| anyhow!("send failed: {e}"))?;
                }
                Ok(())
            })
        })
        .collect();

    // Drop the original Sender so the channel closes once the clones do;
    // otherwise the loop below would block forever waiting for more senders.
    drop(tx);

    // Blocking, idiomatic consumption: iterate until every Sender is gone.
    let mut received = 0;
    for (id, item) in rx {
        println!("producer {id} sent {item}");
        received += 1;
    }

    for producer in producers {
        producer.join().map_err(|_| anyhow!("producer panicked"))??;
    }
    assert_eq!(received, 9);
    Ok(())
}
```

For the common "block, but wake up periodically to do maintenance" case, reach
for [`Receiver::recv_timeout`] rather than polling. Save [`Receiver::try_recv`]
for the situation it is actually built for: a consumer that runs its *own* loop
and only wants to fold in whatever has arrived, without ever stalling on the
channel. The loop below models that — each iteration does a unit of its own work
and then drains any messages that happen to be ready. [`TryRecvError::Empty`]
means "nothing queued, carry on"; [`TryRecvError::Disconnected`] means every
[`Sender`] is gone.

```rust,edition2021
use anyhow::{anyhow, Result};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    let producer = thread::spawn(move || -> Result<()> {
        for command in ["load", "render", "save"] {
            tx.send(command).map_err(|e| anyhow!("send failed: {e}"))?;
            thread::sleep(Duration::from_millis(10));
        }
        Ok(())
    });

    let mut frame = 0u32;
    loop {
        // The consumer's own work — this is why try_recv beats a blocking
        // recv() here: the loop must keep ticking even with no input.
        frame += 1;
        thread::sleep(Duration::from_millis(3)); // stand-in for real work

        match rx.try_recv() {
            Ok(command) => println!("frame {frame}: handling {command}"),
            Err(TryRecvError::Empty) => continue,
            Err(TryRecvError::Disconnected) => break,
        }
    }

    producer.join().map_err(|_| anyhow!("producer panicked"))??;
    println!("ran {frame} frames");
    Ok(())
}
```

[`channel`]: https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html
[`mpsc`]: https://doc.rust-lang.org/std/sync/mpsc/index.html
[`Receiver`]: https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html
[`Receiver::recv_timeout`]: https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html#method.recv_timeout
[`Receiver::try_recv`]: https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html#method.try_recv
[`Sender`]: https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html
[`Sender::send`]: https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html#method.send
[`TryRecvError::Disconnected`]: https://doc.rust-lang.org/std/sync/mpsc/enum.TryRecvError.html#variant.Disconnected
[`TryRecvError::Empty`]: https://doc.rust-lang.org/std/sync/mpsc/enum.TryRecvError.html#variant.Empty
