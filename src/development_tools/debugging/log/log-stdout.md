## Log to stdout instead of stderr

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

Creates a custom logger configuration using the [`Builder::target`] to set the target of the log output to [`Target::Stdout`].

```rust,edition2018

use env_logger::{Builder, Target};

fn main() {
    Builder::new()
        .target(Target::Stdout)
        .init();

    log::error!("This error has been printed to Stdout");
}
```

[`Builder::target`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html#method.target
[`Target::Stdout`]: https://docs.rs/env_logger/*/env_logger/fmt/enum.Target.html
