## Generate random numbers

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

Generates random numbers with help of random-number generator [`rand::Rng`] obtained via
[`rand::rng()`]. Each thread has an initialized generator. Integers are uniformly distributed over the
range of the type, and floating point numbers are uniformly distributed from 0 up to but not
including 1.

```rust
{{#include ../../../crates/algorithms/randomness/src/bin/rand.rs }}
```

[`rand::Rng`]: https://docs.rs/rand/0.9/rand/trait.Rng.html
[`rand::rng()`]: https://docs.rs/rand/0.9/rand/fn.rng.html
