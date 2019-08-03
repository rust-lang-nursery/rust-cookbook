## 英数字からランダムなパスワードを作成する

[![rand-badge]][rand] [![cat-os-badge]][cat-os]

[`Alphanumeric`]を用いた、長さを与えて`A-Z,a-z, 0-9`のランダムなASCII文字を生成するサンプル

```rust
extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

    println!("{}", rand_string);
}
```

[`Alphanumeric`]: https://docs.rs/rand/*/rand/distributions/struct.Alphanumeric.html
