## Generate random values of a custom type

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

Randomly generates a tuple `(i32, bool, f64)` and variable of user defined type `Point`. Implements
the [`Distribution`] trait on type Point for [`Standard`] in order to allow random generation.

```rust
{{#include ../../../crates/algorithms/randomness/src/bin/custom.rs }}
```

[`Distribution`]: https://docs.rs/rand/latest/rand/distr/trait.Distribution.html
[`Standard`]: https://docs.rs/rand/latest/rand/distr/struct.Standard.html
