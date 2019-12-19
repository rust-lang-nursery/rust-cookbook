## Log a debug message to the console

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

The `log` crate provides logging utilities. The `env_logger` crate configures
logging via an environment variable.  The [`log::debug!`] macro works like other
[`std::fmt`] formatted strings.

```rust,edition2018

fn execute_query(query: &str) {
    log::debug!("Executing query: {}", query);
}

fn main() {
    env_logger::init();

    execute_query("DROP TABLE students");
}
```

No output prints when running this code. By default, the
log level is `error`, and any lower levels are dropped.

Set the `RUST_LOG` environment variable to print the message:

```
$ RUST_LOG=debug cargo run
```

Cargo prints debugging information then the
following line at the very end of the output:

```
DEBUG:main: Executing query: DROP TABLE students
```

[`log::debug!`]: https://docs.rs/log/*/log/macro.debug.html
[`std::fmt`]: https://doc.rust-lang.org/std/fmt/
