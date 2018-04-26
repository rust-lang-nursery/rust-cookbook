[ex-rand-dist]: #ex-rand-dist
<a name="ex-rand-dist"></a>

## Generate random numbers with given distribution

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

By default, random numbers are generated with [uniform distribution].
To generate numbers with other distributions you instantiate a
distribution, then sample from that distribution using
[`IndependentSample::ind_sample`] with help of a random-number
generator [`rand::Rng`].

The [distributions available are documented here][rand-distributions]. An example using the
[`Normal`] distribution is shown below.

```rust
extern crate rand;

use rand::distributions::{Normal, IndependentSample};

fn main() {
  let mut rng = rand::thread_rng();

  // mean 2, standard deviation 3:
  let normal = Normal::new(2.0, 3.0);
  let v = normal.ind_sample(&mut rng);
  println!("{} is from a N(2, 9) distribution", v)
}
```

[`IndependentSample::ind_sample`]: https://doc.rust-lang.org/rand/0.4/rand/distributions/trait.IndependentSample.html#tymethod.ind_sample
[`Normal`]: https://doc.rust-lang.org/rand/0.4/rand/distributions/normal/struct.Normal.html
[`rand::Rng`]: https://doc.rust-lang.org/rand/0.4/rand/trait.Rng.html
[rand-distributions]: https://doc.rust-lang.org/rand/0.4/rand/distributions/index.html
[uniform distribution]: https://en.wikipedia.org/wiki/Uniform_distribution_(continuous)
