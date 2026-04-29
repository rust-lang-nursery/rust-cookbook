## Read multiple files concurrently

[![tokio-badge]][tokio] [![cat-filesystem-badge]][cat-filesystem]

Reads several files at once instead of one after another. Each
read is spawned onto the runtime with [`tokio::task::JoinSet`], so
the syscalls overlap and total wall time is roughly the slowest
read rather than the sum of all of them. This is the pattern to
reach for when collecting log shards, manifest files, or chunked
uploads from disk.

The example seeds a temp directory with five files, then reads
them concurrently and reports each file's size and the total. The
async equivalent of [`std::fs::read`] is [`tokio::fs::read`].

```rust,no_run
{{#include ../../../crates/file/async_fs/src/bin/concurrent_read.rs:1:33}}
```

[`std::fs::read`]: https://doc.rust-lang.org/std/fs/fn.read.html
[`tokio::fs::read`]: https://docs.rs/tokio/*/tokio/fs/fn.read.html
[`tokio::task::JoinSet`]: https://docs.rs/tokio/*/tokio/task/struct.JoinSet.html
