## Actor Pattern with Tokio (Handle/Actor/Message)

[![tokio-badge]][tokio] [![cat-concurrency-badge]][cat-concurrency] [![cat-rust-patterns-badge]][cat-rust-patterns]

A common pattern for managing shared mutable state without `Arc<Mutex<T>>` is the
actor pattern, popularized by [Alice Ryhl]. An **actor** is a struct that owns its
data and runs inside a single spawned task. A clonable **handle** holds an
[`mpsc::Sender`] and is the public API. **Messages** are an enum describing every
command the actor can process.

Because only one task ever accesses the data, there is no locking. Request–response
pairs use a [`oneshot`] channel embedded in the message variant.

```rust,edition2021
{{#include ../../../crates/concurrency/actor/src/bin/actor_pattern.rs::153 }}
```

[Alice Ryhl]: https://ryhl.io/blog/actors-with-tokio/
[`mpsc::Sender`]: https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Sender.html
[`oneshot`]: https://docs.rs/tokio/latest/tokio/sync/oneshot/index.html
