## 配列の要素を並列に変換する

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

この例ではRustで並列処理をするためのライブラリである`rayon`クレートを使っています。`rayon`は任意の並列でイテレータブルなデータ型に対して[`par_iter_mu`]メソッドを提供します。

```rust
extern crate rayon;

use rayon::prelude::*;

fn main() {
    let mut arr = [0, 7, 9, 11];
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr);
}
```

[`par_iter_mut`]: https://docs.rs/rayon/*/rayon/iter/trait.IntoParallelRefMutIterator.html#tymethod.par_iter_mut
