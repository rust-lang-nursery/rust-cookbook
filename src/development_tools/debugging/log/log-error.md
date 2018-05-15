<a name="ex-log-error"></a>
## Log an error message to the console

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

```rust
#[macro_use]
extern crate log;
extern crate env_logger;

fn execute_query(_query: &str) -> Result<()> {
    // Do the thing, or maybe not

    bail!("I'm afraid I can't do that")
}

fn main() {
    env_logger::init();

    let response = execute_query("DROP TABLE students");
    if let Err(err) = response {
        // Log the error message and continue
        error!("Failed to execute query: {}", err);
    }
}
```

Run this code with `cargo run` and you should see the following line:

```
DEBUG:main: Failed to execute query: I'm afraid I can't do that
```
