
# TOML

[![toml][toml-badge]][toml]

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

Parse TOML into your own structs using the `serde` crate:

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

# License

MIT/Apache-2.0

<!-- Links -->
[toml-badge]: https://img.shields.io/crates/v/rustc-serialize.svg?label=toml
[toml]: http://alexcrichton.com/toml-rs/toml/

