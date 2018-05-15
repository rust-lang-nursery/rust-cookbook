<a name="ex-log-stdout"></a>
## Log to stdout instead of stderr

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

Creates a custom logger configuration using the [`LogBuilder::target`] to set the target of the log output to [`Target::Stdout`].

```rust
# #[macro_use]
# extern crate error_chain;
#[macro_use]
extern crate log;
extern crate env_logger;

use env_logger::{LogBuilder, LogTarget};
#
# error_chain! {
#     foreign_links {
#         SetLogger(log::SetLoggerError);
#     }
# }

fn run() -> Result<()> {
    LogBuilder::new()
        .target(LogTarget::Stdout)
        .init()?;

    error!("This error has been printed to Stdout");
    Ok(())
}
#
# quick_main!(run);
```

[`LogBuilder::target`]: https://doc.rust-lang.org/log/env_logger/struct.Builder.html#method.target
[`Target::Stdout`]: https://doc.rust-lang.org/log/env_logger/enum.Target.html
