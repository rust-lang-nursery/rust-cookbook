# Builder Approach
[![tokio-badge]][tokio] [![std-badge]][std]

[`tokio::main`] works well for most programs, but it makes all the decisions for you.
If you need control over how the runtime is set up, use the [`Builder`] instead.

Think of Builder as a recipe. Each method you call adds an instruction, how many threads to
use, what to call them, how much memory to give each one. Nothing actually happens until 
you call [`.build()`] at the end, which is when the runtime is created and ready to run.

In this example, the recipe sets up 4 worker threads, gives them the name "thread-one",
and sets a stack size of 3MB each.

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
[`.build()`]: https://docs.rs/tokio/*/tokio/runtime/struct.Builder.html#method.build
