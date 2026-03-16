## Implement a Custom `Future` (Pin, Waker, Poll)

[![std-badge]][std] [![cat-concurrency-badge]][cat-concurrency] [![cat-rust-patterns-badge]][cat-rust-patterns]

When you need low-level control: a custom hardware driver, a specialized timer,
or a zero-allocation protocol parser—you can implement [`Future`] by hand
instead of relying on `async`/`await`.

Every custom future must handle three concepts:

| Concept | Why it matters |
|---------|----------------|
| **[`Pin<&mut Self>`]** | Guarantees the future won't move in memory after the first poll. This is critical for self-referential futures (e.g., those holding borrows across `.await` points). If your struct contains only ordinary fields (no self-references), the compiler auto-implements `Unpin` and pinning is effectively a no-op. |
| **[`Poll::Pending`] / [`Poll::Ready`]** | `Pending` tells the executor "not done yet," while `Ready(value)` completes the future. |
| **[`cx.waker()`]** | A handle the executor gives you. You *must* call `wake()` at some point after returning `Pending`, or the executor will never poll the future again and it will hang. |

The example below builds a simple `Delay` future that resolves after a
deadline.  It has no self-referential fields, so it is `Unpin` automatically—
pinning costs nothing.  A production timer would register with a reactor;
here we call `wake_by_ref()` immediately so the executor re-polls us.

```rust
{{#include ../../../crates/concurrency/custom_future/src/bin/custom_future.rs::71}}
```

[`Future`]: https://doc.rust-lang.org/std/future/trait.Future.html
[`Pin<&mut Self>`]: https://doc.rust-lang.org/std/pin/struct.Pin.html
[`Poll::Pending`]: https://doc.rust-lang.org/std/task/enum.Poll.html#variant.Pending
[`Poll::Ready`]: https://doc.rust-lang.org/std/task/enum.Poll.html#variant.Ready
[`cx.waker()`]: https://doc.rust-lang.org/std/task/struct.Context.html#method.waker
