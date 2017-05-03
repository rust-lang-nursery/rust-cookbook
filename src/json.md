# JSON
[![badge]][`serde_json`]

## JSON implementation in Rust:

The [`serde_json`] crate provides a [`from_str`] function to parse a `&str` of
JSON into a type of the caller's choice.

Unstructured JSON can be parsed into a universal [`serde_json::Value`] type that
is able to represent any valid JSON data.

The example below shows a `&str` of JSON being parsed and then compared to what
we expect the parsed value to be. The expected value is declared using the
[`json!`] macro.

```rust
#[macro_use]
extern crate serde_json;

use serde_json::Value;

fn main() {
    let j = r#"{
                 "userid": 103609,
                 "verified": true,
                 "access_privelages": [
                   "user",
                   "admin"
                 ]
               }"#;

    let parsed: Value = serde_json::from_str(j).unwrap();

    let expected = json!({
        "userid": 103609,
        "verified": true,
        "access_privelages": [
            "user",
            "admin"
        ]
    });

    assert_eq!(parsed, expected);
}
```

# License

MIT/Apache-2.0

<!-- Links -->

[badge]: https://img.shields.io/crates/v/serde_json.svg?label=serde_json
[`serde_json`]: https://docs.serde.rs/serde_json/
[`from_str`]: https://docs.serde.rs/serde_json/fn.from_str.html
[`serde_json::Value`]: https://docs.serde.rs/serde_json/enum.Value.html
[`json!]: https://docs.serde.rs/serde_json/macro.json.html
