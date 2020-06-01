# Pass data between two threads

[![crossbeam-badge]][crossbeam] [![cat-concurrency-badge]][cat-concurrency]

This example demonstrates the use of [crossbeam-channel] in a single producer, single
consumer (SPSC) setting. We build off the [ex-crossbeam-spawn] example by using
[`crossbeam::scope`] and [`Scope::spawn`] to manage the producer thread. Data is
exchanged between the two threads using a [`crossbeam_channel::unbounded`]
channel, meaning there is no limit to the number of storeable messages. The
producer thread sleeps for half a second in between messages.

```rust,edition2018

use std::{thread, time};
use crossbeam_channel::unbounded;

fn main() {
    let (snd, rcv) = unbounded();
    let n_msgs = 5;
    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd.send(i).unwrap();
                thread::sleep(time::Duration::from_millis(100));
            }
        });
    }).unwrap();
    for _ in 0..n_msgs {
        let msg = rcv.recv().unwrap();
        println!("Received {}", msg);
    }
}
```

[crossbeam-channel]: https://docs.rs/crate/crossbeam-channel/
[ex-crossbeam-spawn]: concurrency/threads.html#spawn-a-short-lived-thread
[`crossbeam::scope`]: https://docs.rs/crossbeam/*/crossbeam/fn.scope.html
[`Scope::spawn`]: https://docs.rs/crossbeam/*/crossbeam/thread/struct.Scope.html#method.spawn
[`crossbeam_channel::unbounded`]: https://docs.rs/crossbeam-channel/*/crossbeam_channel/fn.unbounded.html
