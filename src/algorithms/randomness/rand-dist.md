## Generate random numbers with given distribution

[![rand_distr-badge]][rand_distr] [![cat-science-badge]][cat-science]

By default, random numbers in the `rand` crate have
[uniform distribution]. The [`rand_distr`] crate provides
other kinds of distributions. To use them, you instantiate
a distribution, then sample from that distribution using
[`Distribution::sample`] with help of a random-number
generator [`rand::Rng`].

The [distributions available are documented here][rand-distributions].
An example using the [`Normal`] distribution is shown below.

```rust,edition2018
use rand::Rng;
use rand_distr::{Distribution, LogNormal, Normal};

fn main() {
    let mut rng = rand::rng();
    let normal = Normal::new(2.0, 3.0)
        .expect("Failed to create normal distribution");
    let log_normal = LogNormal::new(1.0, 0.5)
        .expect("Failed to create log-normal distribution");

    let v = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v);
    let v = log_normal.sample(&mut rng);
    println!("{} is from an ln N(1, 0.25) distribution", v);
}
```

[`Distribution::sample`]: https://docs.rs/rand/0.9/rand/distr/trait.Distribution.html#tymethod.sample
[`Normal`]: https://docs.rs/rand_distr/*/rand_distr/struct.Normal.html
[`rand::Rng`]: https://docs.rs/rand/0.9/rand/trait.Rng.html
[`rand_distr`]: https://docs.rs/rand_distr/*/rand_distr/index.html
[rand-distributions]: https://docs.rs/rand_distr/*/rand_distr/index.html

[uniform distribution]: https://en.wikipedia.org/wiki/Uniform_distribution_(continuous)
