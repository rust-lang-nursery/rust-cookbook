# Encoding

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Serialize and deserialize unstructured JSON][ex-json-value] | [![serde-json-badge]][serde-json] | [![cat-encoding-badge]][cat-encoding] |
| [Deserialize a TOML configuration file][ex-toml-config] | [![toml-badge]][toml] | [![cat-encoding-badge]][cat-encoding] |

[ex-json-value]: #ex-json-value
<a name="ex-json-value"></a>
## Serialize and deserialize unstructured JSON

[![serde-json-badge]][serde-json] [![cat-encoding-badge]][cat-encoding]

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

[`json!`]: https://docs.serde.rs/serde_json/macro.json.html

```rust
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate error_chain;

use serde_json::Value;

error_chain! {
    foreign_links {
        Json(serde_json::Error);
    }
}

fn run() -> Result<()> {
    let j = r#"{
                 "userid": 103609,
                 "verified": true,
                 "access_privileges": [
                   "user",
                   "admin"
                 ]
               }"#;

    let parsed: Value = serde_json::from_str(j)?;

    let expected = json!({
        "userid": 103609,
        "verified": true,
        "access_privileges": [
            "user",
            "admin"
        ]
    });

    assert_eq!(parsed, expected);

    Ok(())
}

quick_main!(run);
```

[ex-toml-config]: #ex-toml-config
<a name="ex-toml-config"></a>
## Deserialize a TOML configuration file

[![toml-badge]][toml] [![cat-encoding-badge]][cat-encoding]

Parse some TOML into a universal `toml::Value` that is able to represent any
valid TOML data.

```rust
extern crate toml;

#[macro_use]
extern crate error_chain;

use toml::Value;

error_chain! {
    foreign_links {
        Toml(toml::de::Error);
    }
}

fn run() -> Result<()> {
    let toml_content = r#"
          [package]
          name = "your_package"
          version = "0.1.0"
          authors = ["You! <you@example.org>"]

          [dependencies]
          serde = "1.0"
          "#;

    let package_info: Value = toml::from_str(toml_content)?;

    assert_eq!(package_info["dependencies"]["serde"].as_str(), Some("1.0"));
    assert_eq!(package_info["package"]["name"].as_str(), Some("your_package"));

    Ok(())
}

quick_main!(run);
```

Parse TOML into your own structs using Serde:

[![serde-json-badge]][serde-json] [![toml-badge]][toml] [![cat-encoding-badge]][cat-encoding]

```rust
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate toml;

#[macro_use]
extern crate error_chain;

use std::collections::HashMap;

#[derive(Deserialize)]
struct Config {
    package: Package,
    dependencies: HashMap<String, String>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}

error_chain! {
    foreign_links {
        Toml(toml::de::Error);
    }
}

fn run() -> Result<()> {
    let toml_content = r#"
          [package]
          name = "your_package"
          version = "0.1.0"
          authors = ["You! <you@example.org>"]

          [dependencies]
          serde = "1.0"
          "#;

    let package_info: Config = toml::from_str(toml_content)?;

    assert_eq!(package_info.package.name, "your_package");
    assert_eq!(package_info.package.version, "0.1.0");
    assert_eq!(package_info.package.authors, vec!["You! <you@example.org>"]);
    assert_eq!(package_info.dependencies["serde"], "1.0");

    Ok(())
}

quick_main!(run);
```

<!-- Categories -->

[cat-encoding-badge]: https://img.shields.io/badge/-encoding-red.svg
[cat-encoding]: https://crates.io/categories/encoding

<!-- Crates -->

[serde-json-badge]: https://img.shields.io/crates/v/serde_json.svg?label=serde_json
[serde-json]: https://docs.serde.rs/serde_json/
[toml-badge]: https://img.shields.io/crates/v/toml.svg?label=toml
[toml]: https://docs.rs/toml/
