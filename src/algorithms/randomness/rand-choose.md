## Create random passwords from a set of user-defined characters

[![rand-badge]][rand] [![cat-os-badge]][cat-os]

Randomly generates a string of given length ASCII characters with custom user-defined bytestring,
with [`random_range`].

```rust
{{#include ../../../crates/algorithms/randomness/src/bin/choose.rs }}
```

[`random_range`]: https://docs.rs/rand/0.9/rand/trait.Rng.html#method.random_range
