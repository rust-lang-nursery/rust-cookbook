# Logging

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Log a debug message to the console][ex-log-debug] | [![log-badge]][log] [![env_logger-badge]][env_logger] | [![cat-debugging-badge]][cat-debugging] |
| [Log an error message to the console][ex-log-error] | [![log-badge]][log] [![env_logger-badge]][env_logger] | [![cat-debugging-badge]][cat-debugging] |
| [Enable log levels per module][ex-log-mod] | [![log-badge]][log] [![env_logger-badge]][env_logger] | [![cat-debugging-badge]][cat-debugging] |
| [Log to stdout instead of stderr][ex-log-stdout] | [![log-badge]][log] [![env_logger-badge]][env_logger] | [![cat-debugging-badge]][cat-debugging] |
| [Log messages with a custom logger][ex-log-custom-logger] | [![log-badge]][log] | [![cat-debugging-badge]][cat-debugging] |
| [Include timestamp in log messages][ex-log-timestamp] | [![log-badge]][log] [![env_logger-badge]][env_logger] [![chrono-badge]][chrono] | [![cat-debugging-badge]][cat-debugging] |
| [Log to the Unix syslog][ex-log-syslog] | [![log-badge]][log] [![syslog-badge]][syslog] | [![cat-debugging-badge]][cat-debugging] |
| [Log messages to a custom location][ex-log-custom] | [![log-badge]][log] | [![cat-debugging-badge]][cat-debugging] |

[ex-log-debug]: #ex-log-debug
<a name="ex-log-debug"></a>
## Log a debug message to the console

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

The `log` crate provides logging utilities. The `env_logger` crate configures
logging via an environment variable.

```rust
#[macro_use]
extern crate log;
extern crate env_logger;
#
# #[macro_use]
# extern crate error_chain;
#
# error_chain! {
#     foreign_links {
#         SetLogger(log::SetLoggerError);
#     }
# }

fn execute_query(query: &str) {
    debug!("Executing query: {}", query);

    // then do the thing
}

fn run() -> Result<()> {
    env_logger::init()?;

    execute_query("DROP TABLE students");

    Ok(())
}
#
# quick_main!(run);
```

If you run this code, you'll notice that no output is printed. By default, the
log level is `error`, and any lower levels are dropped.

We can change that easily by setting the `RUST_LOG` environment variable:

```
$ RUST_LOG=debug cargo run
```

After running this, you'll likely see a pile of logs from cargo, as well as the
following line at the very end of the output:

```
DEBUG:main: Executing query: DROP TABLE students
```

[ex-log-error]: #ex-log-error
<a name="ex-log-error"></a>
## Log an error message to the console

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

```rust
#[macro_use]
extern crate log;
extern crate env_logger;
#
# #[macro_use]
# extern crate error_chain;
#
# error_chain! {
#     foreign_links {
#         SetLogger(log::SetLoggerError);
#     }
# }

fn execute_query(_query: &str) -> Result<()> {
    // Do the thing, or maybe not

    bail!("I'm afraid I can't do that")
}

fn run() -> Result<()> {
    env_logger::init()?;

    let response = execute_query("DROP TABLE students");
    if let Err(err) = response {
        // Log the error message and continue
        error!("Failed to execute query: {}", err);
    }

    Ok(())
}
#
# quick_main!(run);
```

Run this code with `cargo run` and you should see the following line:

```
DEBUG:main: Failed to execute query: I'm afraid I can't do that
```

[ex-log-mod]: #ex-log-mod
<a name="ex-log-mod"></a>
## Enable log levels per module

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

Creates two modules `foo` and nested `foo::bar` with logging directives
controlled separately with [`RUST_LOG`] environmental variable.

```rust
# #[macro_use]
# extern crate error_chain;
#[macro_use]
extern crate log;
extern crate env_logger;

mod foo {
    mod bar {
        pub fn run() {
            warn!("[bar] warn");
            info!("[bar] info");
            debug!("[bar] debug");
        }
    }

    pub fn run() {
        warn!("[foo] warn");
        info!("[foo] info");
        debug!("[foo] debug");
        bar::run();
    }
}
#
# error_chain! {
#     foreign_links {
#         SetLogger(log::SetLoggerError);
#     }
# }

fn run() -> Result<()> {
    env_logger::init()?;
    warn!("[root] warn");
    info!("[root] info");
    debug!("[root] debug");
    foo::run();

    Ok(())
}
#
# quick_main!(run);
```

[`env_logger`][env_logger] output is controlled by [`RUST_LOG`] environmental
variable on per module basis with comma separated entries in format `path::to::module=log_level`.
Running the `test` application as follows:

```bash
RUST_LOG="warn,test::foo=info,test::foo::bar=debug" ./test
```

Sets the default [`log::LogLevel`] to `warn`, module's `foo` and module's `foo::bar`
respectively to `info` and `debug`. The output is:

```bash
WARN:test: [root] warn
WARN:test::foo: [foo] warn
INFO:test::foo: [foo] info
WARN:test::foo::bar: [bar] warn
INFO:test::foo::bar: [bar] info
DEBUG:test::foo::bar: [bar] debug
```

[ex-log-stdout]: #ex-log-stdout
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

[ex-log-custom-logger]: #ex-log-custom-logger
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

[ex-log-timestamp]: #ex-log-timestamp
<a name="ex-log-timestamp"></a>
## Include timestamp in log messages

[![log-badge]][log] [![env_logger-badge]][env_logger] [![chrono-badge]][chrono] [![cat-debugging-badge]][cat-debugging]

Creates a custom logger configuration with [`LogBuilder`].
Each log entry calls [`Local::now`] to get the current [`DateTime`] in local timezone and uses [`DateTime::format`] with [`strftime::specifiers`] to format a timestamp used in the final log.

The example calls [`LogBuilder::format`] to set a closure which formats each
message text with timestamp, [`LogRecord::level`] and body ([`LogRecord::args`]).
Subsequently calls [`LogBuilder::parse`] which parses `MY_APP_LOG` environmental variable contents in the form of [`RUST_LOG`] syntax.
Finally calls [`LogBuilder::init`] to initialize the logger.
All these steps are normally done internally by [`env_logger::init`].

```rust
# #[macro_use]
# extern crate error_chain;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;

use chrono::Local;
use env_logger::LogBuilder;
use std::env;
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

[ex-log-syslog]: #ex-log-syslog
<a name="ex-log-syslog"></a>
## Log to the Unix syslog

[![log-badge]][log] [![syslog-badge]][syslog] [![cat-debugging-badge]][cat-debugging]

Logs messages to [UNIX syslog]. Initializes logger backend
with [`syslog::init`]. [`syslog::Facility`] indicates type of program submitting log, [`log::LogLevelFilter`] denotes allowed log verbosity
and `Option<&str>` holds optional application name.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
#[macro_use]
extern crate log;
# #[cfg(target_os = "linux")]
extern crate syslog;

use log::LogLevelFilter;
# #[cfg(target_os = "linux")]
use syslog::Facility;
#
# error_chain! {
#     foreign_links {
#         SetLogger(log::SetLoggerError);
#     }
# }

# #[cfg(target_os = "linux")]
fn run() -> Result<()> {
    syslog::init(Facility::LOG_USER,
                 LogLevelFilter::Debug,
                 Some("My app name"))?;
    debug!("this is a debug {}", "message");
    error!("this is an error!");
    Ok(())
}
#
# #[cfg(not(target_os = "linux"))]
# fn run() -> Result<()> {
#     Ok(())
# }
#
# quick_main!(run);
```

[ex-log-custom]: #ex-log-custom
<a name="ex-log-custom"></a>
## Log messages to a custom location

[![log-badge]][log] [![log4rs-badge]][log4rs] [![cat-debugging-badge]][cat-debugging]

Configures log to be output into custom location with [log4rs]. [log4rs] can use either an external YAML file or a programmatically constructed configuration.

Firstly creates the log configuration with [`log4rs::append::file::FileAppender`]
using a custom pattern from [`log4rs::encode::pattern`].

Secondly assigns it to the [`log4rs::config::Config`] which has a root appender that uses the previously created `logfile` appender. Subsequently sets the default [`log::LogLevelFilter`] so that any logs with `Info` level or higher will be sent to the logger.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
#[macro_use]
extern crate log;
extern crate log4rs;

use log::LogLevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
#
# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#         LogConfig(log4rs::config::Errors);
#         SetLogger(log::SetLoggerError);
#     }
# }

fn run() -> Result<()> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(LogLevelFilter::Info))?;

    log4rs::init_config(config)?;

    info!("Hello, world!");

    Ok(())
}
#
# quick_main!(run);
```

<!-- Categories -->

[cat-debugging-badge]: https://badge-cache.kominick.com/badge/debugging--x.svg?style=social
[cat-debugging]: https://crates.io/categories/debugging

<!-- Crates -->

[chrono-badge]: https://badge-cache.kominick.com/crates/v/chrono.svg?label=chrono
[chrono]: https://docs.rs/chrono/
[env_logger-badge]: https://badge-cache.kominick.com/crates/v/env_logger.svg?label=env_logger
[env_logger]: https://docs.rs/env_logger/
[log-badge]: https://badge-cache.kominick.com/crates/v/log.svg?label=log
[log4rs-badge]: https://badge-cache.kominick.com/crates/v/log4rs.svg?label=log4rs
[log4rs]: https://docs.rs/log4rs/
[log]: https://docs.rs/log/
[syslog-badge]: https://badge-cache.kominick.com/crates/v/syslog.svg?label=syslog
[syslog]: https://docs.rs/syslog/

<!-- Reference -->

[UNIX syslog]: https://www.gnu.org/software/libc/manual/html_node/Overview-of-Syslog.html
[`DateTime::format`]: https://docs.rs/chrono/*/chrono/datetime/struct.DateTime.html#method.format
[`DateTime`]: https://docs.rs/chrono/*/chrono/datetime/struct.DateTime.html
[`Local::now`]: https://docs.rs/chrono/*/chrono/offset/local/struct.Local.html#method.now
[`LogBuilder`]: https://doc.rust-lang.org/log/env_logger/struct.Builder.html
[`LogBuilder::format`]: https://doc.rust-lang.org/log/env_logger/struct.LogBuilder.html#method.format
[`LogBuilder::init`]: https://doc.rust-lang.org/log/env_logger/struct.LogBuilder.html#method.init
[`LogBuilder::parse`]: https://doc.rust-lang.org/log/env_logger/struct.LogBuilder.html#method.parse
[`LogBuilder::target`]: https://doc.rust-lang.org/log/env_logger/struct.Builder.html#method.target
[`LogRecord::args`]: https://doc.rust-lang.org/log/log/struct.LogRecord.html#method.args
[`LogRecord::level`]: https://doc.rust-lang.org/log/log/struct.LogRecord.html#method.level
[`RUST_LOG`]: https://doc.rust-lang.org/log/env_logger/#enabling-logging
[`env_logger::init`]: https://doc.rust-lang.org/log/env_logger/fn.init.html
[`Target::Stdout`]: https://doc.rust-lang.org/log/env_logger/enum.Target.html
[`log4rs::append::file::FileAppender`]: https://docs.rs/log4rs/*/log4rs/append/file/struct.FileAppender.html
[`log4rs::config::Config`]: https://docs.rs/log4rs/*/log4rs/config/struct.Config.html
[`log4rs::encode::pattern`]: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
[`log::LogLevelFilter`]: https://doc.rust-lang.org/log/log/enum.LogLevelFilter.html
[`log::LogLevel`]: https://doc.rust-lang.org/log/log/enum.LogLevel.html
[`log::Log`]: https://doc.rust-lang.org/log/log/trait.Log.html
[`log::set_logger`]: https://doc.rust-lang.org/log/log/fn.set_logger.html
[`strftime::specifiers`]: https://docs.rs/chrono/*/chrono/format/strftime/index.html#specifiers
[`syslog::Facility`]: https://docs.rs/syslog/*/syslog/enum.Facility.html
[`syslog::init`]: https://docs.rs/syslog/*/syslog/fn.init.html
