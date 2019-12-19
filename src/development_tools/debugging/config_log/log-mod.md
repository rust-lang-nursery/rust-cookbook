## Enable log levels per module

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

Creates two modules `foo` and nested `foo::bar` with logging directives
controlled separately with [`RUST_LOG`] environmental variable.

```rust,edition2018

mod foo {
    mod bar {
        pub fn run() {
            log::warn!("[bar] warn");
            log::info!("[bar] info");
            log::debug!("[bar] debug");
        }
    }

    pub fn run() {
        log::warn!("[foo] warn");
        log::info!("[foo] info");
        log::debug!("[foo] debug");
        bar::run();
    }
}

fn main() {
    env_logger::init();
    log::warn!("[root] warn");
    log::info!("[root] info");
    log::debug!("[root] debug");
    foo::run();
}
```

[`RUST_LOG`] environment variable controls [`env_logger`][env_logger] output.
Module declarations take comma separated entries formatted like
`path::to::module=log_level`. Run the `test` application as follows:

```bash
RUST_LOG="warn,test::foo=info,test::foo::bar=debug" ./test
```

Sets the default [`log::Level`] to `warn`, module `foo` and module `foo::bar`
to `info` and `debug`.

```bash
WARN:test: [root] warn
WARN:test::foo: [foo] warn
INFO:test::foo: [foo] info
WARN:test::foo::bar: [bar] warn
INFO:test::foo::bar: [bar] info
DEBUG:test::foo::bar: [bar] debug
```

[`log::Level`]: https://docs.rs/log/*/log/enum.Level.html
[`RUST_LOG`]: https://docs.rs/env_logger/*/env_logger/#enabling-logging
