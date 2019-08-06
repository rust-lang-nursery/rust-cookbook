## 与えられた述語を使って並列にアイテムを探す

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

This example uses [`rayon::find_any`] and [`par_iter`] to search a vector in
parallel for an element satisfying the predicate in the given closure.

If there are multiple elements satisfying the predicate defined in the closure
argument of [`rayon::find_any`], `rayon` returns the first one found, not
necessarily the first one.

Also note that the argument to the closure is a reference to a reference
(`&&x`). See the discussion on [`std::find`] for additional details.

この例では与えられたクロージャーの条件を満たす要素を並列的にベクタから探すために[`rayon::find_any`]と[`par_iter`]を使います。

[`rayon::find_any`]の引数で定義されたクロージャを満たす複数の要素があれば、`rayon`は最初に見つかった値を返します。必ずしも先頭の値ではありません。(並列的に探すため)

```rust
extern crate rayon;

use rayon::prelude::*;

fn main() {
    let v = vec![6, 2, 1, 9, 3, 8, 11];

    let f1 = v.par_iter().find_any(|&&x| x == 9);
    let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
    let f3 = v.par_iter().find_any(|&&x| x > 8);

    assert_eq!(f1, Some(&9));
    assert_eq!(f2, Some(&8));
    assert!(f3 > Some(&8));
}
```

[`par_iter`]: https://docs.rs/rayon/*/rayon/iter/trait.IntoParallelRefIterator.html#tymethod.par_iter
[`rayon::find_any`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.find_any
[`std::find`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find
