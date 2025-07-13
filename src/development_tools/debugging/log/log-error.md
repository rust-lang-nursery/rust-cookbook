## Log an error message to the console

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

Proper error handling considers exceptions exceptional.  Here, an error logs
to stderr with `log`'s convenience macro [`log::error!`].

```rust,edition2018
fn execute_query(_query: &str) -> Result<(), &'static str> {
    Err("I'm afraid I can't do that")
}

fn main() {
    env_logger::init();

    let response = execute_query("DROP TABLE students");
    if let Err(err) = response {
        log::error!("Failed to execute query: {}", err);
    }
}
```

[`log::error!`]: https://docs.rs/log/*/log/macro.error.html
