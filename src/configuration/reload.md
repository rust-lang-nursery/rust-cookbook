## Reload configuration when the file changes

[![config-badge]][config] [![notify-badge]][notify] [![cat-config-badge]][cat-config] [![cat-filesystem-badge]][cat-filesystem]

A long-running service can pick up edits to its config file without a restart by
watching the file and rebuilding its settings when it changes. This combines
[`config`][config] for loading with [`notify`][notify] for change detection.
[Watch a directory for changes](../file/watch.html) covers the watching primitives.

[`new_debouncer`] collapses the burst of filesystem events an editor emits on
save into a single notification, so the config is reloaded once per change rather
than several times. Each notification triggers a fresh `load`; a parse error is
logged and the previous settings are kept, so a malformed edit never takes down
the running program.

```rust,no_run
{{#include ../../crates/configuration/config/src/bin/reload.rs::54}}
```

[`new_debouncer`]: https://docs.rs/notify-debouncer-full/*/notify_debouncer_full/fn.new_debouncer.html

{{#include ../links.md}}
