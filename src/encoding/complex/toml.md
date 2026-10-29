## Deserialize a TOML configuration file

[![toml-badge]][toml] [![serde-badge]][serde] [![cat-encoding-badge]][cat-encoding]

Parse some TOML into a universal `toml::Value` that is able to represent any
valid TOML data.

> This recipe requires the [`derive`] feature flag of `serde` to be enabled in `Cargo.toml`.

```rust
{{#include ../../../crates/encoding/toml/src/bin/value.rs }}
```

Parse TOML into your own structs using [Serde].

```rust
{{#include ../../../crates/encoding/toml/src/bin/structs.rs }}
```
[`derive`]: https://docs.rs/crate/serde/*/features#derive
