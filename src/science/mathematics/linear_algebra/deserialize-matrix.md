## (De)-Serialize a Matrix
[![nalgebra-badge]][nalgebra] [![serde-json-badge]][serde-json] [![cat-science-badge]][cat-science]

Serialize and deserialize a matrix to and from JSON. Serialization is taken care of
by [`serde_json::to_string`] and [`serde_json::from_str`] performs deserialization.

Note that serialization followed by deserialization gives back the original matrix.

> This recipe requires the [`serde-serialize`] feature flag of `nalgebra` to be enabled in `Cargo.toml`.
The Rust Playground does not enable this feature, so the recipe has no Run button.

```rust,noplayground
{{#include ../../../../crates/science/linear_algebra/src/bin/deserialize_matrix.rs }}
```

[`serde_json::to_string`]: https://docs.rs/serde_json/*/serde_json/fn.to_string.html
[`serde_json::from_str`]: https://docs.rs/serde_json/*/serde_json/fn.from_str.html
[`serde-serialize`]: https://docs.rs/crate/nalgebra/*/features#serde-serialize
