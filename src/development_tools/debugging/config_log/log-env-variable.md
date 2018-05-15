<a name="ex-log-env-variable"></a>
## Use a custom environment variable to set up logging

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

Logging is configured with [`LogBuilder`].

[`LogBuilder::parse`] parses `MY_APP_LOG`
environmental variable contents in the form of [`RUST_LOG`] syntax.
Then [`LogBuilder::init`] initializes the logger.
All these steps are normally done internally by [`env_logger::init`].

```rust
# #[macro_use]
# extern crate error_chain;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use env_logger::LogBuilder;

# error_chain! {
#     foreign_links {
#         EnvLogger(log::SetLoggerError);
#     }
# }
#
fn run() -> Result<()> {
    LogBuilder::new()
        .parse(&env::var("MY_APP_LOG").unwrap_or_default())
        .init()?;

    info!("informational message");
    warn!("warning message");
    error!("this is an error {}", "message");

    Ok(())
}
#
# quick_main!(run);
```

[`env_logger::init`]: https://doc.rust-lang.org/log/env_logger/fn.init.html
[`LogBuilder`]: https://doc.rust-lang.org/log/env_logger/struct.Builder.html
[`LogBuilder::init`]: https://doc.rust-lang.org/log/env_logger/struct.LogBuilder.html#method.init
[`LogBuilder::parse`]: https://doc.rust-lang.org/log/env_logger/struct.LogBuilder.html#method.parse
[`RUST_LOG`]: https://doc.rust-lang.org/log/env_logger/#enabling-logging
