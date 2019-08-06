## 並列にベクタをソートする

[![rayon-badge]][rayon] [![rand-badge]][rand] [![cat-concurrency-badge]][cat-concurrency]

この例では文字列のベクタを並列にソートします。

空の文字列のベクタを割り当てます。`par_iter_mut().for_each`は並列にランダムな値を入れます。また、enumerableなデータ型をソートするためのいつかのオプションが存在していて[`par_sort_unstable`]は通常、[stable sorting]アルゴリズムより早いです。

```rust
extern crate rand;
extern crate rayon;

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rayon::prelude::*;

fn main() {
  let mut vec = vec![String::new(); 100_000];
  vec.par_iter_mut().for_each(|p| {
    let mut rng = thread_rng();
    *p = (0..5).map(|_| rng.sample(&Alphanumeric)).collect()
  });
  vec.par_sort_unstable();
}
```

[`par_sort_unstable`]: https://docs.rs/rayon/*/rayon/slice/trait.ParallelSliceMut.html#method.par_sort_unstable
[multiple options]: https://docs.rs/rayon/*/rayon/slice/trait.ParallelSliceMut.html
[stable sorting]: https://docs.rs/rayon/*/rayon/slice/trait.ParallelSliceMut.html#method.par_sort
