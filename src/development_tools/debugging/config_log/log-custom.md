## Log messages to a custom location

[![log-badge]][log] [![log4rs-badge]][log4rs] [![cat-debugging-badge]][cat-debugging]

[log4rs] configures log output to a custom location. [log4rs] can use either an
external YAML file or a builder configuration.

Create the log configuration with [`log4rs::append::file::FileAppender`]. An
appender defines the logging destination.  The configuration continues with
encoding using a custom pattern from [`log4rs::encode::pattern`].
Assigns the configuration to [`log4rs::config::Config`] and sets the default
[`log::LevelFilter`].

```rust,no_run
{{#include ../../../../crates/development_tools/debugging/log4rs/src/bin/log-custom.rs }}
```

[`log4rs::append::file::FileAppender`]: https://docs.rs/log4rs/*/log4rs/append/file/struct.FileAppender.html
[`log4rs::config::Config`]: https://docs.rs/log4rs/*/log4rs/config/runtime/struct.Config.html
[`log4rs::encode::pattern`]: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
[`log::LevelFilter`]: https://docs.rs/log/*/log/enum.LevelFilter.html
