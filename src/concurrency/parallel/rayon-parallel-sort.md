## Sort a vector in parallel

[![rayon-badge]][rayon] [![rand-badge]][rand] [![cat-concurrency-badge]][cat-concurrency]

This example will sort in parallel a vector of Strings.

Allocate a vector of empty Strings. `par_iter_mut().for_each` populates random
values in parallel.  Although [multiple options]
exist to sort an enumerable data type, [`par_sort_unstable`]
is usually faster than [stable sorting] algorithms.

```rust,edition2018
use rand::Rng;
use rayon::prelude::*;

fn main() {
    let mut vec = vec![0; 1_000_000];
    rand::thread_rng().fill(&mut vec[..]);

    vec.par_sort_unstable();

    let first = vec.first().unwrap();
    let last = vec.last().unwrap();
    assert!(first <= last);
}
```

[`par_sort_unstable`]: https://docs.rs/rayon/*/rayon/slice/trait.ParallelSliceMut.html#method.par_sort_unstable
[multiple options]: https://docs.rs/rayon/*/rayon/slice/trait.ParallelSliceMut.html
[stable sorting]: https://docs.rs/rayon/*/rayon/slice/trait.ParallelSliceMut.html#method.par_sort
