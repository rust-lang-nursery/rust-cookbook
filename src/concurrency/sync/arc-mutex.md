## Guard compound state with `Arc<Mutex<T>>`

[![std-badge]][std] [![cat-concurrency-badge]][cat-concurrency]

When the shared value is more than a single atomic-sized field — a struct whose
fields must stay consistent with one another — a [`Mutex`] is the right tool.
[`Arc`] gives every thread shared ownership of the same value; [`Mutex`] ensures
only one thread mutates it at a time. For a plain integer counter, prefer
[`AtomicUsize`] (above); reach for `Arc<Mutex<T>>` once an update touches several
fields that must change together.

Each thread acquires the lock through [`Mutex::lock`], updates *both* fields of
the shared `Stats`, and releases the lock when the returned [`MutexGuard`] drops.
Holding one lock across the whole update is what keeps `count` and `total` from
ever being observed out of sync — something separate atomics could not
guarantee. A poisoned lock (a thread panicked while holding it) surfaces as an
error rather than a panic. When the shared state grows into its own long-lived
component, the [Actor Pattern] is the usual scale-up path.

```rust,edition2021
use anyhow::{anyhow, Result};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Default)]
struct Stats {
    count: u64,
    total: u64,
}

fn main() -> Result<()> {
    let stats = Arc::new(Mutex::new(Stats::default()));
    let mut handles = Vec::new();

    for value in 1..=10 {
        let stats = Arc::clone(&stats);
        let handle = thread::spawn(move || -> Result<()> {
            let mut stats = stats.lock().map_err(|_| anyhow!("mutex poisoned"))?;
            // Both fields update under one lock, so count and total can
            // never be observed out of step with each other.
            stats.count += 1;
            stats.total += value;
            Ok(())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().map_err(|_| anyhow!("thread panicked"))??;
    }

    let stats = stats.lock().map_err(|_| anyhow!("mutex poisoned"))?;
    println!("{:?}", *stats);
    assert_eq!(stats.count, 10);
    assert_eq!(stats.total, 55);
    Ok(())
}
```

[Actor Pattern]: actor.html
[`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[`AtomicUsize`]: https://doc.rust-lang.org/std/sync/atomic/type.AtomicUsize.html
[`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[`Mutex::lock`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html#method.lock
[`MutexGuard`]: https://doc.rust-lang.org/std/sync/struct.MutexGuard.html
