## Create random passwords from a set of user-defined characters

[![rand-badge]][rand] [![cat-os-badge]][cat-os]

Randomly generates a string of given length ASCII characters with custom
user-defined bytestring, with [`random_range`].

```rust,edition2018
use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789)(*&^%$#@!~";
const PASSWORD_LEN: usize = 30;

fn main() {
    let mut rng = rand::rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            char::from(CHARSET[idx])
        })
        .collect();

    println!("{:?}", password);
}
```

[`random_range`]: https://docs.rs/rand/0.9/rand/trait.Rng.html#method.random_range
