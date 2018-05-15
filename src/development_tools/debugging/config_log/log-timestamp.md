<a name="ex-log-timestamp"></a>
## Include timestamp in log messages

[![log-badge]][log] [![env_logger-badge]][env_logger] [![chrono-badge]][chrono] [![cat-debugging-badge]][cat-debugging]

Creates a custom logger configuration with [`LogBuilder`].
Each log entry calls [`Local::now`] to get the current [`DateTime`] in local timezone and uses [`DateTime::format`] with [`strftime::specifiers`] to format a timestamp used in the final log.

The example calls [`LogBuilder::format`] to set a closure which formats each
message text with timestamp, [`LogRecord::level`] and body ([`LogRecord::args`]).

```rust
# #[macro_use]
# extern crate error_chain;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;

use std::env;
use env_logger::LogBuilder;
use chrono::Local;

#
# error_chain! {
#     foreign_links {
#         SetLogger(log::SetLoggerError);
#     }
# }

fn run() -> Result<()> {
    LogBuilder::new()
        .format(|record| {
                    format!("{} [{}] - {}",
                            Local::now().format("%Y-%m-%dT%H:%M:%S"),
                            record.level(),
                            record.args())
                })
        .parse(&env::var("MY_APP_LOG").unwrap_or_default())
        .init()?;

    warn!("warn");
    info!("info");
    debug!("debug");
    Ok(())
}
#
# quick_main!(run);
```
Calling `MY_APP_LOG="info" cargo run` will result in similar output:
```
2017-05-22T21:57:06 [WARN] - warn
2017-05-22T21:57:06 [INFO] - info
```

[`DateTime::format`]: https://docs.rs/chrono/*/chrono/datetime/struct.DateTime.html#method.format
[`DateTime`]: https://docs.rs/chrono/*/chrono/datetime/struct.DateTime.html
[`Local::now`]: https://docs.rs/chrono/*/chrono/offset/local/struct.Local.html#method.now
[`LogBuilder`]: https://doc.rust-lang.org/log/env_logger/struct.Builder.html
[`LogBuilder::format`]: https://doc.rust-lang.org/log/env_logger/struct.LogBuilder.html#method.format
[`LogRecord::args`]: https://doc.rust-lang.org/log/log/struct.LogRecord.html#method.args
[`LogRecord::level`]: https://doc.rust-lang.org/log/log/struct.LogRecord.html#method.level
[`strftime::specifiers`]: https://docs.rs/chrono/*/chrono/format/strftime/index.html#specifiers
