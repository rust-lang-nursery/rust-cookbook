[ex-rayon-parallel-sort]: #ex-rayon-parallel-sort
<a name="ex-rayon-parallel-sort"></a>
## Sort a vector in parallel

[![rayon-badge]][rayon] [![rand-badge]][rand] [![cat-concurrency-badge]][cat-concurrency]

This example will sort in parallel a vector of Strings.

[1] We start by preallocating a vector of empty Strings, so we can mutate the information in parallel later,
to populate the vector with random Strings.

[2] `par_iter_mut().for_each` takes a closure and applies it in parallel on all the elements of the vector.<br/>
[3] Inside the passed closure we modify the element in the vector with a 5 character-long String, random generated.

[4] We have [multiple options] to sort an Iterable data type, we chose here to use [`par_sort_unstable`]
because it is usually faster than [stable sorting] algorithms which `rayon` also supports.

```rust
extern crate rand;
extern crate rayon;

use rand::Rng;
use rayon::prelude::*;

fn main() {
    // [1]
    let mut vec = vec![String::new(); 100_000];

    // [2]
    vec.par_iter_mut().for_each(|p| {
        // [3]
        *p = rand::weak_rng().gen_ascii_chars().take(5).collect()
    });

    // [4]
    vec.par_sort_unstable();
}
```

[`par_sort_unstable`]: https://docs.rs/rayon/*/rayon/slice/trait.ParallelSliceMut.html#method.par_sort_unstable
[multiple options]: https://docs.rs/rayon/*/rayon/slice/trait.ParallelSliceMut.html
[stable sorting]: https://docs.rs/rayon/*/rayon/slice/trait.ParallelSliceMut.html#method.par_sort
