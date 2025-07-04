## Generate random numbers

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

Generates random numbers with help of random-number
generator [`rand::Rng`] obtained via [`rand::rng`]. Each thread has an
initialized generator. Integers are uniformly distributed over the range of the
type, and floating point numbers are uniformly distributed from 0 up to but not
including 1.

```rust,edition2018
extern crate rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen();
    println!("Random number: {}", random_number);
}
```

[`rand::Rng`]: https://docs.rs/rand/*/rand/trait.Rng.html
[`rand::rng`]: https://docs.rs/rand/*/rand/fn.rng.html
