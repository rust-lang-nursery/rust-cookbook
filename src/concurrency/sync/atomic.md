## Lock-free counter with `AtomicUsize`

[![std-badge]][std] [![cat-concurrency-badge]][cat-concurrency]

For a simple shared counter, an [`AtomicUsize`] avoids the overhead of a
[`Mutex`] entirely. Atomic operations such as [`AtomicUsize::fetch_add`] complete
as a single indivisible step, so concurrent increments never lose updates and no
lock is required.

Every atomic operation takes an [`Ordering`] that controls how the compiler and
CPU may reorder surrounding memory accesses. [`Ordering::Relaxed`] guarantees the
atomicity of the operation itself but imposes no ordering relative to other
memory operations — perfect for a standalone counter. [`Ordering::SeqCst`]
(sequentially consistent) is the strongest, establishing a single global order
across all `SeqCst` operations; reach for it when an atomic guards access to
*other* data and you need those accesses to be visible in a predictable order.

This counter needs neither. The increments are [`Ordering::Relaxed`] because only
their atomicity matters, and the final read is `Relaxed` too: [`JoinHandle::join`]
already establishes a happens-before edge, so once every thread has joined the
load is guaranteed to observe all of their increments. Using `SeqCst` for that
load would only suggest an ordering requirement that does not exist.

```rust,edition2021
use anyhow::{anyhow, Result};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() -> Result<()> {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                // Relaxed is enough: we only care that the count is exact.
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for handle in handles {
        handle.join().map_err(|_| anyhow!("thread panicked"))?;
    }

    // join() above synchronizes the threads, so a Relaxed load sees the total.
    let total = counter.load(Ordering::Relaxed);
    println!("total: {}", total);
    assert_eq!(total, 10_000);
    Ok(())
}
```

[`AtomicUsize`]: https://doc.rust-lang.org/std/sync/atomic/type.AtomicUsize.html
[`AtomicUsize::fetch_add`]: https://doc.rust-lang.org/std/sync/atomic/type.AtomicUsize.html#method.fetch_add
[`JoinHandle::join`]: https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.join
[`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[`Ordering`]: https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html
[`Ordering::Relaxed`]: https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html#variant.Relaxed
[`Ordering::SeqCst`]: https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html#variant.SeqCst
