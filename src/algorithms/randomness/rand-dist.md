## 分布を指定して乱数を生成する

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

乱数はデフォルトで一様分布になっています。他の分布で乱数を生成するには、分布をインスタンス化して[`rand::Rng`]と[`Distribution::sample`]を使用してその分布から値を生成します。

[使用可能な分布はこちらでドキュメント化されています。][rand-distributions]
正規分布を使用した例を下記に記します。

```rust
extern crate rand;

use rand::distributions::{Normal, Distribution};

fn main() {
  let mut rng = rand::thread_rng();
  let normal = Normal::new(2.0, 3.0);
  let v = normal.sample(&mut rng);
  println!("{} is from a N(2, 9) distribution", v)
}
```

[`Distribution::sample`]: https://docs.rs/rand/*/rand/distributions/trait.Distribution.html#tymethod.sample
[`Normal`]: https://docs.rs/rand/*/rand/distributions/normal/struct.Normal.html
[`rand::Rng`]: https://docs.rs/rand/*/rand/trait.Rng.html
[rand-distributions]: https://docs.rs/rand/*/rand/distributions/index.html

[uniform distribution]: https://en.wikipedia.org/wiki/Uniform_distribution_(continuous)
