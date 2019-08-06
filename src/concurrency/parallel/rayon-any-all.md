## コレクションの要素の一部または全部が特定の述語に一致するかどうかを並列にテストする。

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

この例では並列処理に対応した[`std::any`], [`std::all`]である[`rayon::any`]と[`rayon::all`]を使っています。[`rayon::any`]はイテレータのいずれかのデータが述語と一致するか並列にチェックし、見つかるとすぐにリターンします。[`rayon::all`]は一致しない要素が見つかるとすぐにリターンします。

```rust
extern crate rayon;

use rayon::prelude::*;

fn main() {
    let mut vec = vec![2, 4, 6, 8];

    assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(!vec.par_iter().any(|n| *n > 8 ));
    assert!(vec.par_iter().all(|n| *n <= 8 ));

    vec.push(9);

    assert!(vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(!vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(vec.par_iter().any(|n| *n > 8 ));
    assert!(!vec.par_iter().all(|n| *n <= 8 )); 
}
```

[`rayon::all`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.all
[`rayon::any`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.any
[`std::all`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all
[`std::any`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
