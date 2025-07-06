## Use a custom environment variable to set up logging

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

[`Builder`] configures logging.

[`Builder::from_env`] parses `MY_APP_LOG`
environment variable contents in the form of [`RUST_LOG`] syntax.
Then, [`Builder::init`] initializes the logger.

```rust,edition2018
extern crate log;
extern crate env_logger;
use env_logger::Builder;

fn main() {
    Builder::from_env("MY_APP_LOG").init();

    log::info!("informational message");
    log::warn!("warning message");
    log::error!("this is an error {}", "message");
}
```

[`Builder`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html
[`Builder::from_env`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html#method.from_env
[`Builder::init`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html#method.init
[`RUST_LOG`]: https://docs.rs/env_logger/*/env_logger/#enabling-logging
