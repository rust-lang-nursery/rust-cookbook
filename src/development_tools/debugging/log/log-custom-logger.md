## Log messages with a custom logger

[![log-badge]][log] [![cat-debugging-badge]][cat-debugging]

Implements a custom logger `ConsoleLogger` which prints to stdout.
In order to use the logging macros, `ConsoleLogger` implements
the [`log::Log`] trait and [`log::set_logger`] installs it.

```rust
# #[macro_use]
# extern crate error_chain;
#[macro_use]
extern crate log;

use log::{Record, Level, Metadata, LevelFilter};

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
  fn enabled(&self, metadata: &Metadata) -> bool {
     metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("Rust says: {} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
#
# error_chain! {
#     foreign_links {
#         SetLogger(log::SetLoggerError);
#     }
# }

fn run() -> Result<()> {
    log::set_logger(&CONSOLE_LOGGER)?;
    log::set_max_level(LevelFilter::Info);

    info!("hello log");
    warn!("warning");
    error!("oops");
    Ok(())
}
#
# quick_main!(run);
```

[`log::Log`]: https://docs.rs/log/*/log/trait.Log.html
[`log::set_logger`]: https://docs.rs/log/*/log/fn.set_logger.html
