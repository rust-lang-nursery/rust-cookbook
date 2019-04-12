## Create random passwords from a set of user-defined characters

[![rand-badge]][rand] [![cat-os-badge]][cat-os]

Randomly generates a string of given length ASCII characters with custom user-defined bytestring, with [`choose`].

```rust
extern crate rand;

use rand::seq::SliceRandom;

fn main() {
    const CHARSET: &[u8] =  b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";

    let mut rng = rand::thread_rng();
    let password: Option<String> = (0..30)
        .map(|_| Some(*CHARSET.choose(&mut rng)? as char))
        .collect();

    println!("{:?}", password);
}
```

[`choose`]: https://docs.rs/rand/*/rand/trait.Rng.html#method.choose
