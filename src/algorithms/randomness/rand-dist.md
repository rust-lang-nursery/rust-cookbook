## Generate random numbers with given distribution

[![rand_distr-badge]][rand_distr] [![cat-science-badge]][cat-science]

By default, random numbers in the `rand` crate have
[uniform distribution]. The [`rand_distr`] crate provides
other kinds of distrubutions. To use them, you instantiate
a distribution, then sample from that distribution using
[`Distribution::sample`] with help of a random-number
generator [`rand::Rng`].

The [distributions available are documented here][rand-distributions].
An example using the [`Normal`] distribution is shown below.

```rust,no_run
extern crate rand_distr;
extern crate rand;

use rand_distr::{Distribution, Normal, NormalError};

fn main() -> Result<(), NormalError> {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(2.0, 3.0)?;
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v);
    Ok(())
}
```

[`Distribution::sample`]: https://docs.rs/rand/*/rand/distributions/trait.Distribution.html#tymethod.sample
[`Normal`]: https://docs.rs/rand_distr/*/rand_distr/struct.Normal.html
[`rand::Rng`]: https://docs.rs/rand/*/rand/trait.Rng.html
[`rand_distr`]: https://docs.rs/rand_distr/*/rand_distr/index.html
[rand-distributions]: https://docs.rs/rand_distr/*/rand_distr/index.html

[uniform distribution]: https://en.wikipedia.org/wiki/Uniform_distribution_(continuous)
