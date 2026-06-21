## Coordinate thread phases with `Barrier`

[![std-badge]][std] [![cat-concurrency-badge]][cat-concurrency]

A [`Barrier`] synchronizes a fixed number of threads at a common point. Each
thread calls [`Barrier::wait`]; the call blocks until exactly that many threads
have reached it, then releases them all together. This guarantees every thread
finishes phase 1 before any thread begins phase 2.

The barrier is constructed for three threads, so each of the three workers blocks
in [`Barrier::wait`] until its peers catch up.

```rust,edition2021
use anyhow::{anyhow, Result};
use std::sync::{Arc, Barrier};
use std::thread;

fn main() -> Result<()> {
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = Vec::new();

    for id in 0..3 {
        let barrier = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            println!("thread {} finished phase 1", id);
            // No thread proceeds until all three have arrived.
            barrier.wait();
            println!("thread {} starting phase 2", id);
        }));
    }

    for handle in handles {
        handle.join().map_err(|_| anyhow!("thread panicked"))?;
    }
    Ok(())
}
```

[`Barrier`]: https://doc.rust-lang.org/std/sync/struct.Barrier.html
[`Barrier::wait`]: https://doc.rust-lang.org/std/sync/struct.Barrier.html#method.wait
