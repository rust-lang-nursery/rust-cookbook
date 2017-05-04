# Encoding

| Recipe | Crates |
|--------|--------|
| [Serialize and deserialize unstructured JSON][ex-json-value] | [![serde-json-badge]][serde-json] |
| [Deserialize a TOML configuration file][ex-toml-config] | [![toml-badge]][toml] |

[ex-json-value]: #ex-json-value
<a name="ex-json-value"></a>
## Serialize and deserialize unstructured JSON

[![serde-json-badge]][serde-json]

The [`serde_json`] crate provides a [`from_str`] function to parse a `&str` of
JSON into a type of the caller's choice.

[`serde_json`]: https://docs.serde.rs/serde_json/
[`from_str`]: https://docs.serde.rs/serde_json/fn.from_str.html

Unstructured JSON can be parsed into a universal [`serde_json::Value`] type that
is able to represent any valid JSON data.

[`serde_json::Value`]: https://docs.serde.rs/serde_json/enum.Value.html

The example below shows a `&str` of JSON being parsed and then compared to what
we expect the parsed value to be. The expected value is declared using the
[`json!`] macro.

[`json!]: https://docs.serde.rs/serde_json/macro.json.html

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

[ex-toml-config]: #ex-toml-config
<a name="ex-toml-config"></a>
## Deserialize a TOML configuration file

[![toml-badge]][toml]

Parse TOML into a `toml::Value` and then operate on it:

```rust
extern crate toml;

fn main() {
    let toml_source = "
        [package]
        name = \"your package!\"
        version = \"0.1.0\"
        authors = [\"You! <you@example.org>\"]

        [dependencies]
        cool = \"0.2.1\"";

    let package_info = toml_source.parse::<toml::Value>().unwrap();

    assert_eq!(package_info["dependencies"]["cool"].as_str(), Some("0.2.1"));
    assert_eq!(package_info["package"]["name"].as_str(), Some("your package!"));
}
```

Parse TOML into your own structs using Serde:

```rust
extern crate toml;

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[derive(Deserialize)]
struct Config {
    package: Package,
    dependencies: std::collections::HashMap<String, String>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}

fn main() {
    let toml_source = "
        [package]
        name = \"your package!\"
        version = \"0.1.0\"
        authors = [\"You! <you@example.org>\"]

        [dependencies]
        cool = \"0.2.1\"";

    let package_info : Config = toml::from_str(toml_source).unwrap();

    assert_eq!(package_info.package.name, "your package!");
    assert_eq!(package_info.package.version, "0.1.0");
    assert_eq!(package_info.package.authors, vec!["You! <you@example.org>"]);
    assert_eq!(package_info.dependencies["cool"], "0.2.1");
}
```

<!-- Crates -->

[serde-json-badge]: https://img.shields.io/crates/v/serde_json.svg?label=serde_json
[serde-json]: https://docs.serde.rs/serde_json/
[toml-badge]: https://img.shields.io/crates/v/toml.svg?label=toml
[toml]: https://docs.rs/toml/
