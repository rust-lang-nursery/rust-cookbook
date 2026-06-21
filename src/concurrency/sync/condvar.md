## Signal a waiting thread with `Condvar`

[![std-badge]][std] [![cat-concurrency-badge]][cat-concurrency]

A [`Condvar`] (condition variable) lets a thread sleep until another thread
signals that some shared state has changed, without busy-waiting. It is always
paired with a [`Mutex`] that protects the state being watched.

The waiting thread locks the mutex, then loops while the condition is false,
releasing the lock and sleeping inside [`Condvar::wait`]. The signaling thread
locks the same mutex, updates the state, drops the guard, and calls
[`Condvar::notify_one`] to wake the waiter. The `while` loop guards against
spurious wakeups: the waiter re-checks the condition every time it wakes.

Dropping the guard *before* notifying is a small but worthwhile habit: if the
lock is still held when [`Condvar::notify_one`] fires, the woken thread wakes
only to block again immediately on re-acquiring the mutex. Releasing first lets
it proceed straight away. The result is correct either way — only the hand-off is
tidier.

```rust,edition2021
use anyhow::{anyhow, Result};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() -> Result<()> {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    let signaler = Arc::clone(&pair);
    let worker = thread::spawn(move || -> Result<()> {
        let (lock, cvar) = &*signaler;
        let mut ready = lock.lock().map_err(|_| anyhow!("mutex poisoned"))?;
        *ready = true;
        // Release the lock before signaling so the woken thread can proceed
        // immediately instead of blocking to re-acquire it.
        drop(ready);
        cvar.notify_one();
        Ok(())
    });

    let (lock, cvar) = &*pair;
    let mut ready = lock.lock().map_err(|_| anyhow!("mutex poisoned"))?;
    while !*ready {
        ready = cvar.wait(ready).map_err(|_| anyhow!("mutex poisoned"))?;
    }
    println!("worker signaled readiness");

    worker.join().map_err(|_| anyhow!("worker panicked"))??;
    Ok(())
}
```

[`Condvar`]: https://doc.rust-lang.org/std/sync/struct.Condvar.html
[`Condvar::notify_one`]: https://doc.rust-lang.org/std/sync/struct.Condvar.html#method.notify_one
[`Condvar::wait`]: https://doc.rust-lang.org/std/sync/struct.Condvar.html#method.wait
[`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
