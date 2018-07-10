## Use a custom environment variable to set up logging

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

[`Builder`] configures logging.

[`Builder::parse`] parses `MY_APP_LOG`
environment variable contents in the form of [`RUST_LOG`] syntax.
Then, [`Builder::init`] initializes the logger.
All these steps are normally done internally by [`env_logger::init`].

```rust
#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use env_logger::Builder;

fn main() {
    Builder::new()
        .parse(&env::var("MY_APP_LOG").unwrap_or_default())
        .init();

    info!("informational message");
    warn!("warning message");
    error!("this is an error {}", "message");
}
```

[`env_logger::init`]: https://docs.rs/env_logger/*/env_logger/fn.init.html
[`Builder`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html
[`Builder::init`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html#method.init
[`Builder::parse`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html#method.parse
[`RUST_LOG`]: https://docs.rs/env_logger/*/env_logger/#enabling-logging
