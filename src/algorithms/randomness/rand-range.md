## Generate random numbers within a range

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

Generates a random value within half-open `[0, 10)` range (not including `10`) with [`Rng::random_range`].

```rust
{{#include ../../../crates/algorithms/randomness/src/bin/random_range.rs }}
```

[`Uniform`] can obtain values with [uniform distribution]. This has the same effect, but may be
faster when repeatedly generating numbers in the same range.

```rust
{{#include ../../../crates/algorithms/randomness/src/bin/uniform.rs }}
```

[`Rng::random_range`]: https://docs.rs/rand/0.9/rand/trait.Rng.html#method.random_range
[`Uniform`]: https://docs.rs/rand_distr/latest/rand_distr/struct.Uniform.html
