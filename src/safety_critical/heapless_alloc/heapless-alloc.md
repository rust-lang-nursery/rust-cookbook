## Deterministic Memory with Heapless Collections

[![heapless-badge]][heapless] [![cat-no-std-badge]][cat-no-std] [![cat-data-structures-badge]][cat-data-structures]

Safety-critical and real-time systems often **forbid the heap** entirely.  The
global allocator is non-deterministic—allocation time varies, and long-running
firmware risks fragmentation and silent OOM failures.

[`heapless`] provides `Vec`, `String`, `Deque`, and map types that store data
on the **stack** (or in a `static`) with a capacity fixed at compile time.
Pushing beyond capacity returns `Err` instead of panicking or allocating,
letting the caller decide how to handle it.

| `std` type | `heapless` equivalent | Guarantee |
|---|---|---|
| `Vec<T>` | `heapless::Vec<T, N>` | At most `N` elements, no allocator |
| `String` | `heapless::String<N>` | At most `N` bytes, no allocator |
| `VecDeque<T>` | `heapless::Deque<T, N>` | Fixed-capacity ring buffer |
| `HashMap<K,V>` | `heapless::IndexMap<K,V,S,N>` | Fixed-capacity hash map |

The example below builds a stack-allocated event log and demonstrates
capacity enforcement—the kind of predictable, constant-memory behavior
required by embedded and real-time systems.

```rust
{{#include ../../../crates/safety_critical/heapless_alloc/src/bin/heapless_alloc.rs::83}}
```

[`heapless`]: https://docs.rs/heapless
