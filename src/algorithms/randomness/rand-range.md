## Generate random numbers within a range

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

Generates a random value within half-open `[0, 10)` range (not including `10`) with [`Rng::random_range`].

```rust,edition2018
use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    println!("Integer: {}", rng.random_range(0..10));
    println!("Float: {}", rng.random_range(0.0..10.0));
}
```

[`Uniform`] can obtain values with [uniform distribution].
This has the same effect, but may be faster when repeatedly generating numbers
in the same range.

```rust,edition2018

use rand::distributions::{Distribution, Uniform};

fn main() {
    let mut rng = rand::rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}
```

[`Uniform`]: https://docs.rs/rand/*/rand/distributions/uniform/struct.Uniform.html
[`Rng::random_range`]: https://doc.rust-lang.org/rand/*/rand/trait.Rng.html#method.random_range
[uniform distribution]: https://en.wikipedia.org/wiki/Uniform_distribution_(continuous)
