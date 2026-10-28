## Maintain global mutable state

[![std-badge]][std] [![cat-rust-patterns-badge]][cat-rust-patterns]

Declare global state using a `static` [`Mutex`] (also see [`RwLock`]).
[`Mutex::new`] is a `const fn`, so no lazy initialization is needed for
an empty collection. If the initial value cannot be computed at compile time,
wrap it in a [`LazyLock`], as in `static X: LazyLock<Mutex<T>>`.
The [`Mutex`] wrap ensures
the state cannot be simultaneously accessed by multiple threads, preventing
race conditions. A [`MutexGuard`] must be acquired to read or mutate the
value stored in a [`Mutex`].

```rust,edition2024
use anyhow::{anyhow, Result};
use std::sync::Mutex;

static FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());

fn insert(fruit: &str) -> Result<()> {
    let mut db = FRUIT
        .lock()
        .map_err(|_| anyhow!("Failed to acquire MutexGuard"))?;
    db.push(fruit.to_string());
    Ok(())
}

fn main() -> Result<()> {
    insert("apple")?;
    insert("orange")?;
    insert("peach")?;
    {
        let db = FRUIT
            .lock()
            .map_err(|_| anyhow!("Failed to acquire MutexGuard"))?;

        db.iter()
            .enumerate()
            .for_each(|(i, item)| println!("{}: {}", i, item));
    }
    insert("grape")?;
    Ok(())
}
```

[`LazyLock`]: https://doc.rust-lang.org/std/sync/struct.LazyLock.html
[`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[`Mutex::new`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html#method.new
[`MutexGuard`]: https://doc.rust-lang.org/std/sync/struct.MutexGuard.html
[`RwLock`]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
