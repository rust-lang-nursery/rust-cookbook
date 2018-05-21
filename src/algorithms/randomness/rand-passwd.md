## Create random passwords from a set of alphanumeric characters

[![rand-badge]][rand] [![cat-os-badge]][cat-os]

Randomly generates a string of given length ASCII characters in the range `A-Z, a-z, 0-9`, with [`gen_ascii_chars`].

```rust
extern crate rand;

use rand::{thread_rng, Rng};

fn main() {
    let rand_string: String = thread_rng().gen_ascii_chars().take(30).collect();
    println!("{}", rand_string);
}
```

[`gen_ascii_chars`]: https://docs.rs/rand/0.4/rand/trait.Rng.html#method.gen_ascii_chars
