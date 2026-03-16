## Compile-Time No-Panic Guarantee

[![no-panic-badge]][no-panic] [![cat-no-std-badge]][cat-no-std] [![cat-rust-patterns-badge]][cat-rust-patterns]

In safety-critical systems—automotive braking, medical devices, aerospace—a
`panic!` is a catastrophic failure, not just a crash.  Standard operations like
`array[i]` insert a hidden bounds check that panics on out-of-bounds access.

The [`#[no_panic]`][no-panic] attribute macro makes the **compiler prove** at
link time that a function can never reach a panicking code path.  If it can't
prove it, the build fails.

| Panicking pattern | Safe alternative |
|-------------------|-----------------|
| `slice[i]` | `slice.get(i)` with exhaustive `match` |
| `for i in 0..len { slice[i] }` | `for &v in slice` (iterator) |
| `value.unwrap()` | `match` / `if let` / `?` |
| `a / b` (integer, `b` might be 0) | Check `b` before dividing |

The example below shows three panic-free functions—aggregation, lookup, and
sensor normalization—each proven at compile time by `#[no_panic]`.

```rust
{{#include ../../../crates/safety_critical/no_panic/src/bin/no_panic.rs::64}}
```

[no-panic]: https://docs.rs/no-panic
