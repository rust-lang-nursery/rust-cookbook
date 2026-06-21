## Chain fallible operations with Result

[![std-badge]][std] [![cat-rust-patterns-badge]][cat-rust-patterns]

[`Result`] carries the same family of combinators as [`Option`], letting a
sequence of fallible steps read as a single expression. [`Result::map_err`]
rewrites an error into a domain-specific type so it can cross an API boundary.
[`Result::and_then`] sequences another fallible step, short-circuiting at the
first error. [`Result::map`] transforms a successful value while leaving any
error untouched.

```rust,edition2021
#[derive(Debug)]
enum ConfigError {
    NotANumber(String),
    OutOfRange(u32),
}

fn socket_addr(host: &str, raw_port: &str) -> Result<String, ConfigError> {
    raw_port
        .parse::<u32>()
        // `map_err` rewrites the standard-library error as our own type.
        .map_err(|_| ConfigError::NotANumber(raw_port.to_string()))
        // `and_then` runs another fallible step, short-circuiting on error.
        .and_then(|n| u16::try_from(n).map_err(|_| ConfigError::OutOfRange(n)))
        // `map` transforms the success value, leaving any error untouched.
        .map(|port| format!("{host}:{port}"))
}

fn main() -> Result<(), ConfigError> {
    println!("valid:      {:?}", socket_addr("localhost", "8080"));
    println!("not number: {:?}", socket_addr("localhost", "nope"));
    println!("out of range: {:?}", socket_addr("localhost", "70000"));

    assert_eq!(socket_addr("localhost", "8080")?, "localhost:8080");
    assert!(matches!(
        socket_addr("localhost", "nope"),
        Err(ConfigError::NotANumber(_))
    ));
    assert!(matches!(
        socket_addr("localhost", "70000"),
        Err(ConfigError::OutOfRange(_))
    ));

    Ok(())
}
```
