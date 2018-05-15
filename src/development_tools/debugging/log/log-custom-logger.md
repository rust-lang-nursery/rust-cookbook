<a name="ex-log-custom-logger"></a>
## Log messages with a custom logger

[![log-badge]][log] [![cat-debugging-badge]][cat-debugging]

Implements a custom logger `ConsoleLogger` which prints to stdout.
In order to use the logging macros, `ConsoleLogger` implements
the [`log::Log`] trait and has to be installed via [`log::set_logger`].

```rust
# #[macro_use]
# extern crate error_chain;
#[macro_use]
extern crate log;

use log::{LogRecord, LogLevel, LogMetadata, LogLevelFilter};

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("Rust says: {} - {}", record.level(), record.args());
        }
    }
}
#
# error_chain! {
#     foreign_links {
#         SetLogger(log::SetLoggerError);
#     }
# }

fn run() -> Result<()> {
    log::set_logger(|max_log_level| {
                        max_log_level.set(LogLevelFilter::Info);
                        Box::new(ConsoleLogger)
                    })?;

    info!("hello log");
    warn!("warning");
    error!("oops");
    Ok(())
}
#
# quick_main!(run);
```

[`log::Log`]: https://doc.rust-lang.org/log/log/trait.Log.html
[`log::set_logger`]: https://doc.rust-lang.org/log/log/fn.set_logger.html
