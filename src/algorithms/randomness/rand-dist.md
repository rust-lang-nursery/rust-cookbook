## Generate random numbers with given distribution

[![rand_distr-badge]][rand_distr] [![cat-science-badge]][cat-science]

By default, random numbers in the `rand` crate have [uniform distribution]. The [`rand_distr`] crate
provides other kinds of distributions. To use them, you instantiate a distribution, then sample from
that distribution using [`Distribution::sample`] with help of a random-number generator
[`rand::Rng`].

The [distributions available are documented here][rand-distributions]. An example using the
[`Normal`] distribution is shown below.

```rust
{{#include ../../../crates/algorithms/randomness/src/bin/dist.rs }}
```

[`Distribution::sample`]: https://docs.rs/rand/0.9/rand/distr/trait.Distribution.html#tymethod.sample
[`Normal`]: https://docs.rs/rand_distr/*/rand_distr/struct.Normal.html
[`rand::Rng`]: https://docs.rs/rand/0.9/rand/trait.Rng.html
[`rand_distr`]: https://docs.rs/rand_distr/*/rand_distr/index.html
[rand-distributions]: https://docs.rs/rand_distr/*/rand_distr/index.html

[uniform distribution]: https://en.wikipedia.org/wiki/Uniform_distribution_(continuous)
