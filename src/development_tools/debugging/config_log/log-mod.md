<a name="ex-log-mod"></a>
## Enable log levels per module

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

Creates two modules `foo` and nested `foo::bar` with logging directives
controlled separately with [`RUST_LOG`] environmental variable.

```rust
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

fn main() {
    env_logger::init();
    warn!("[root] warn");
    info!("[root] info");
    debug!("[root] debug");
    foo::run();
}
```

[`env_logger`][env_logger] output is controlled by [`RUST_LOG`] environmental
variable on per module basis with comma separated entries in format `path::to::module=log_level`.
Running the `test` application as follows:

```bash
RUST_LOG="warn,test::foo=info,test::foo::bar=debug" ./test
```

Sets the default [`log::Level`] to `warn`, module's `foo` and module's `foo::bar`
respectively to `info` and `debug`. The output is:

```bash
WARN:test: [root] warn
WARN:test::foo: [foo] warn
INFO:test::foo: [foo] info
WARN:test::foo::bar: [bar] warn
INFO:test::foo::bar: [bar] info
DEBUG:test::foo::bar: [bar] debug
```

[`log::Level`]: https://docs.rs/log/*/log/enum.Level.html
[`RUST_LOG`]: https://doc.rust-lang.org/log/env_logger/#enabling-logging
