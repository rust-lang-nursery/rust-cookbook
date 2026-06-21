## Convert an Option to a Result

[![std-badge]][std] [![cat-rust-patterns-badge]][cat-rust-patterns]

A missing [`Option`] often needs to become a meaningful error so it can be
propagated with `?`. [`Option::ok_or`] turns `Some(v)` into `Ok(v)` and `None`
into `Err(e)` using a supplied error value. [`Option::ok_or_else`] builds that
error lazily from a closure, which is preferable when constructing the error
allocates or is otherwise expensive.

```rust,edition2021
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct MissingKey(String);

fn get_setting(config: &HashMap<String, String>, key: &str) -> Result<String, MissingKey> {
    // `ok_or_else` bridges `Option` to `Result`: `Some(v)` becomes
    // `Ok(v)`, and `None` becomes `Err`, with the error built lazily.
    config
        .get(key)
        .cloned()
        .ok_or_else(|| MissingKey(key.to_string()))
}

fn main() -> Result<(), MissingKey> {
    let mut config = HashMap::new();
    config.insert("host".to_string(), "localhost".to_string());

    let host = get_setting(&config, "host")?;
    println!("host setting: {host}");
    assert_eq!(host, "localhost");

    // Use `ok_or` when the error value is cheap to create up front.
    let port: Result<&String, MissingKey> =
        config.get("port").ok_or(MissingKey("port".to_string()));
    println!("port setting: {port:?}");
    assert_eq!(port, Err(MissingKey("port".to_string())));

    Ok(())
}
```
