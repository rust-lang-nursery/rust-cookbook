# Builder Approach
[![tokio-badge]][tokio] [![std-badge]][std]

[`Builder`] gives you full control over the async runtime configuration before starting it. This 
is used when there is a need to tune thread count, stack size, or thread names instead of accepting
the defaults from #[[`tokio::main`]].

```rust,edition2018
use std::io;
use tokio::runtime::Builder;

async fn fetch_network_request() -> u32 {
    89
}

fn main() -> io::Result<()> {
     let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("thread-one")
        .thread_stack_size(3 * 1024 * 1024)
        .build()?;

    runtime.spawn(async {
        let result = fetch_network_request().await;
        assert_eq!(result, 89);
    });

    Ok(())
}
```

> Add `tokio` to `Cargo.toml` with the [`rt-multi-thread`] feature enabled.
> ```toml
> [dependencies]
> tokio = { version = "*", features = ["rt-multi-thread"] }
> ```

[`Builder`]: https://docs.rs/tokio/*/tokio/runtime/struct.Builder.html
[`rt-multi-thread`]: https://docs.rs/crate/tokio/*/features#rt-multi-thread
[`tokio::main`]: https://docs.rs/tokio/*/tokio/attr.main.html
