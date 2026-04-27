# Structured Concurrency
This section illustrates how to manage multiple running async tasks.

## Join Sets

[![tokio-badge]][tokio]

[`JoinSet`] spawns and tracks a group of async tasks. [`join_next`] waits for the next task to
finish and returns its result. This continues until all tasks are complete. If a JoinSet is dropped
before all tasks complete, all remaining tasks are cancelled automatically

```rust,edition2018
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();

    for i in 0..10 {
        set.spawn(async move { i });
    }

    let mut seen = [false; 10];
    while let Some(res) = set.join_next().await {
        let idx = res.unwrap();
        seen[idx] = true;
    }

    for x in seen {
        assert!(x);
    }
}
```

> Add `tokio` to `Cargo.toml` with the [`macros`] and [`rt`] features enabled.
> ```toml
> [dependencies]
> tokio = { version = "*", features = ["macros", "rt"] }
> ```

{{#include ../links.md}}

[`JoinSet`]: https://docs.rs/tokio/*/tokio/task/struct.JoinSet.html
[`join_next`]: https://docs.rs/tokio/*/tokio/task/struct.JoinSet.html#method.join_next
[`macros`]: https://docs.rs/crate/tokio/*/features#macros
[`rt`]: https://docs.rs/crate/tokio/*/features#rt
