## Concurrent reads with `Arc<RwLock<T>>`

[![std-badge]][std] [![cat-concurrency-badge]][cat-concurrency]

When data is read often and written rarely, [`RwLock`] is a better fit than
[`Mutex`]. It allows any number of concurrent readers *or* a single exclusive
writer. Readers acquire a shared lock with [`RwLock::read`]; a writer acquires an
exclusive lock with [`RwLock::write`] that blocks until every reader has released
its lock.

[`RwLock`] gives no fairness or priority guarantee, and the policy is
platform-dependent. On some targets a steady stream of readers can keep the lock
held continuously and *starve* a waiting writer, so reach for `RwLock` only when
reads genuinely dominate; under heavy contention a plain [`Mutex`] (or a
dedicated fair lock) can be the safer choice.

This recipe spawns several reader threads that observe the shared vector
simultaneously, plus one writer thread that appends to it under an exclusive
lock.

```rust,edition2021
use anyhow::{anyhow, Result};
use std::sync::{Arc, RwLock};
use std::thread;

fn main() -> Result<()> {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = Vec::new();

    for id in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || -> Result<()> {
            let reader = data.read().map_err(|_| anyhow!("lock poisoned"))?;
            println!("reader {} sees {:?}", id, *reader);
            Ok(())
        }));
    }

    let writer_data = Arc::clone(&data);
    handles.push(thread::spawn(move || -> Result<()> {
        let mut writer = writer_data.write().map_err(|_| anyhow!("lock poisoned"))?;
        writer.push(4);
        Ok(())
    }));

    for handle in handles {
        handle.join().map_err(|_| anyhow!("thread panicked"))??;
    }

    let final_state = data.read().map_err(|_| anyhow!("lock poisoned"))?;
    println!("final: {:?}", *final_state);
    Ok(())
}
```

[`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[`RwLock`]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
[`RwLock::read`]: https://doc.rust-lang.org/std/sync/struct.RwLock.html#method.read
[`RwLock::write`]: https://doc.rust-lang.org/std/sync/struct.RwLock.html#method.write
