## Merge multiple providers with figment

[![figment-badge]][figment] [![cat-config-badge]][cat-config]

[`figment`][figment] assembles configuration from independent *providers*. A
provider can be a TOML, JSON, or YAML file, or a set of environment variables.
Figment merges them into a typed value.
It is the configuration library behind [Rocket], and its [`Jail`] makes
configuration code testable without touching the real filesystem or environment.

A [`Figment`] stacks providers with [`merge`]: later providers override keys set
by earlier ones. Here [`Toml::file`] reads the base settings and [`Env::prefixed`]
layers `APP_`-prefixed environment variables on top, so `APP_PORT=9000`
overrides the file's `port`. [`extract`] deserializes the merged result into the
`Config` struct.

```rust,no_run
{{#include ../../crates/configuration/figment/src/bin/multi_provider.rs::25}}
```

Configuration code is awkward to test when it reads real files and environment
variables. [`Jail::expect_with`] runs a closure in a temporary directory with a
sandboxed environment, so a test can create an `App.toml` and set variables that
disappear when the closure returns:

```rust,ignore
{{#include ../../crates/configuration/figment/src/bin/multi_provider.rs:31:47}}
```

[Rocket]: https://rocket.rs/
[`Figment`]: https://docs.rs/figment/*/figment/struct.Figment.html
[`merge`]: https://docs.rs/figment/*/figment/struct.Figment.html#method.merge
[`Toml::file`]: https://docs.rs/figment/*/figment/providers/trait.Format.html#method.file
[`Env::prefixed`]: https://docs.rs/figment/*/figment/providers/struct.Env.html#method.prefixed
[`extract`]: https://docs.rs/figment/*/figment/struct.Figment.html#method.extract
[`Jail`]: https://docs.rs/figment/*/figment/struct.Jail.html
[`Jail::expect_with`]: https://docs.rs/figment/*/figment/struct.Jail.html#method.expect_with

{{#include ../links.md}}
