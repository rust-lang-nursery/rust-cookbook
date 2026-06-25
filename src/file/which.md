# Find an Executable

## Find a binary on the PATH

[![which-badge]][which] [![cat-filesystem-badge]][cat-filesystem]

Locating an executable means walking each directory on the `PATH`. It also means
applying the platform's rules. On Windows that includes the `PATHEXT` extensions
such as `.exe`. The [`which`] crate does exactly what a shell does. It hands back
the absolute path of the first match.

[`which`] resolves a single best match. [`which_all`] yields every match in
precedence order. That helps when a name is shadowed by more than one install. A
name that is not on the `PATH` comes back as an [`Error`]. A caller handles a
missing tool with `?` or a `match` rather than a panic.

```rust,no_run
{{#include ../../crates/file/which/src/bin/find_binary.rs::18 }}
```

[`which`]: https://docs.rs/which/*/which/fn.which.html
[`which_all`]: https://docs.rs/which/*/which/fn.which_all.html
[`Error`]: https://docs.rs/which/*/which/enum.Error.html

{{#include ../links.md}}
