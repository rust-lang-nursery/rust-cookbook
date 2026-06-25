## Watch a directory for changes

[![notify-badge]][notify] [![cat-filesystem-badge]][cat-filesystem]

[`notify`] reports filesystem changes through one API. Under it sits a native
backend. That is inotify on Linux, FSEvents on macOS, kqueue on the BSDs and
`ReadDirectoryChangesW` on Windows. The watcher runs that backend on its own
thread. It hands events back over a [`std::sync::mpsc`] channel. That is the
standard library's multi-producer, single-consumer queue. [`recommended_watcher`]
takes the sending half and forwards each change to it as a
`Result<`[`Event`]`, `[`Error`]`>`. Your thread keeps the receiving half and
reads events by iterating it.

Passing [`RecursiveMode::Recursive`] to [`watch`] follows the entire subtree.
Files created in nested directories are reported too. Every [`Event`] carries an
[`EventKind`] and the affected paths. Matching on the kind keeps creations, data
writes and removals. It discards metadata-only noise. The receiver blocks until
the [`Watcher`] is dropped. A real tool keeps this loop running for its lifetime.
Run it and edit files under the watched directory to watch events stream in.

```rust,no_run
{{#include ../../../crates/file/notify/src/bin/watch.rs::31 }}
```

[`notify`]: https://docs.rs/notify/
[`std::sync::mpsc`]: https://doc.rust-lang.org/std/sync/mpsc/index.html
[`recommended_watcher`]: https://docs.rs/notify/*/notify/fn.recommended_watcher.html
[`Event`]: https://docs.rs/notify/*/notify/event/struct.Event.html
[`EventKind`]: https://docs.rs/notify/*/notify/event/enum.EventKind.html
[`Error`]: https://docs.rs/notify/*/notify/struct.Error.html
[`RecursiveMode::Recursive`]: https://docs.rs/notify/*/notify/enum.RecursiveMode.html
[`watch`]: https://docs.rs/notify/*/notify/trait.Watcher.html#tymethod.watch
[`Watcher`]: https://docs.rs/notify/*/notify/trait.Watcher.html
