## Search items using given predicate in parallel

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

This example uses [`rayon::find_any`] and [`par_iter`] to search a vector in
parallel for an element satisfying the predicate in the given closure.

If there are multiple elements satisfying the predicate defined in the closure
argument of [`rayon::find_any`], `rayon` returns the first one found, not
necessarily the first one.

Also note that the argument to the closure is a reference to a reference
(`&&x`). See the discussion on [`std::find`] for additional details.

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
