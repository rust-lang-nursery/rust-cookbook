## Debounce a burst of file events

[![notify-debouncer-full-badge]][notify-debouncer-full] [![cat-filesystem-badge]][cat-filesystem]

A single logical change often surfaces as several raw events. Editors save by
writing a temporary file and renaming it. Some backends emit separate create,
modify and metadata notifications for one write. Acting on every raw event makes
a watcher rebuild or reload far more often than the user intended.

[`notify-debouncer-full`] wraps a [`notify`] watcher and coalesces events that
land within a time window. [`new_debouncer`] takes that window, an optional tick
rate and a channel. Once a burst settles it delivers one deduplicated
`Vec<`[`DebouncedEvent`]`>` instead of a flood. It also resolves renames and
drops redundant events, so each path appears once per quiet period.

```rust,no_run
{{#include ../../../crates/file/notify/src/bin/debounce.rs::35 }}
```

[`notify`]: https://docs.rs/notify/
[`notify-debouncer-full`]: https://docs.rs/notify-debouncer-full/
[`new_debouncer`]: https://docs.rs/notify-debouncer-full/*/notify_debouncer_full/fn.new_debouncer.html
[`DebouncedEvent`]: https://docs.rs/notify-debouncer-full/*/notify_debouncer_full/struct.DebouncedEvent.html
