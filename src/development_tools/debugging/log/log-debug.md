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
