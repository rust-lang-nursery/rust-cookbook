## Layer defaults, a file, and environment variables

[![config-badge]][config] [![cat-config-badge]][cat-config]

Most applications resolve their configuration from several places: defaults that
ship with the binary, a config file an operator edits, and environment variables
set at deploy time. [`config`][config] builds one typed value from those layers,
where each source overrides the one before it.

[`ConfigBuilder`] collects sources in precedence order. [`set_default`] seeds the
baseline values, [`File`] loads `config.toml` (marked [`required(false)`] so a
missing file falls back to the defaults), and [`Environment`] reads `APP_`-prefixed
variables. `APP_PORT=9090` sets the `port` key. [`build`] merges them and
[`try_deserialize`] produces the typed `Settings`.

```rust,no_run
{{#include ../../crates/configuration/config/src/bin/layered.rs::30}}
```

The bundled `config.toml` supplies the file layer:

```toml
{{#include ../../crates/configuration/config/config.toml}}
```

[`ConfigBuilder`]: https://docs.rs/config/*/config/builder/struct.ConfigBuilder.html
[`set_default`]: https://docs.rs/config/*/config/builder/struct.ConfigBuilder.html#method.set_default
[`File`]: https://docs.rs/config/*/config/struct.File.html
[`required(false)`]: https://docs.rs/config/*/config/struct.File.html#method.required
[`Environment`]: https://docs.rs/config/*/config/struct.Environment.html
[`build`]: https://docs.rs/config/*/config/builder/struct.ConfigBuilder.html#method.build
[`try_deserialize`]: https://docs.rs/config/*/config/struct.Config.html#method.try_deserialize

{{#include ../links.md}}
