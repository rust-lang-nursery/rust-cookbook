## Store per-user configuration with confy

[![confy-badge]][confy] [![cat-config-badge]][cat-config]

A desktop or CLI program that remembers a user's preferences should write them
to the platform's standard config location, not next to the binary.
[`confy`][confy] handles that: it serializes a `Default`-able, `Serialize` +
`Deserialize` struct to the per-user config directory and reads it back, creating
the file from the defaults on first run.

[`confy::load`] takes an application name and resolves `~/.config/<app>/<app>.toml`
on Linux (and the platform equivalent elsewhere); [`get_configuration_file_path`]
returns that location without writing anything. To keep the recipe from touching
your real config directory, it round-trips through an explicit path with
[`load_path`] and [`store_path`] instead.

```rust,no_run
{{#include ../../crates/configuration/confy/src/bin/per_user.rs::33}}
```

[`confy::load`]: https://docs.rs/confy/*/confy/fn.load.html
[`get_configuration_file_path`]: https://docs.rs/confy/*/confy/fn.get_configuration_file_path.html
[`load_path`]: https://docs.rs/confy/*/confy/fn.load_path.html
[`store_path`]: https://docs.rs/confy/*/confy/fn.store_path.html

{{#include ../links.md}}
