# Concurrency

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Mutate the elements of an array in parallel][ex-rayon-iter-mut] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [Spawn a short-lived thread][ex-crossbeam-spawn] | [![crossbeam-badge]][crossbeam] | [![cat-concurrency-badge]][cat-concurrency] |

[ex-rayon-iter-mut]: #ex-rayon-iter-mut
<a name="ex-rayon-iter-mut"></a>
## Mutate the elements of an array in parallel

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

```rust
extern crate rayon;

use rayon::prelude::*;

fn main() {
    let mut arr = [0, 7, 9, 11];

    arr.par_iter_mut().for_each(|p| *p -= 1);

    println!("{:?}", arr);
}
```

The example uses the Rayon crate, which is a data parallelism library for Rust.
Rayon provides the `par_iter_mut()` method for any parallel iterable data type.
It lets us write iterator-like chains that execute in parallel.

[ex-crossbeam-spawn]: #ex-crossbeam-spawn
<a name="ex-crossbeam-spawn"></a>
## Spawn a short-lived thread

[![crossbeam-badge]][crossbeam] [![cat-concurrency-badge]][cat-concurrency]

```rust
extern crate crossbeam;

use std::cmp;

fn main() {
    let arr = &[-4, 1, 10, 25];
    let max = find_max(arr, 0, arr.len());
    assert_eq!(25, max);
}

fn find_max(arr: &[i32], start: usize, end: usize) -> i32 {
    // Perform sequential computation if there are only a few elements.
    const THRESHOLD: usize = 2;
    if end - start <= THRESHOLD {
        return *arr.iter().max().unwrap();
    }

    let mid = start + (end - start) / 2;
    crossbeam::scope(|scope| {
        let left = scope.spawn(|| find_max(arr, start, mid));
        let right = scope.spawn(|| find_max(arr, mid, end));

        cmp::max(left.join(), right.join())
    })
}
```

The example uses `crossbeam` crate, which provides data structures and functions 
for concurrent and parallel programming. [`Scope::spawn`] spawns a new scoped thread that is guaranteed
to terminate before returning from the closure that passed into [`crossbeam::scope`] function, meaning that
you can reference data from the calling function.

<!-- Categories -->

[cat-concurrency-badge]: https://badge-cache.kominick.com/badge/concurrency--x.svg?style=social
[cat-concurrency]: https://crates.io/categories/concurrency

<!-- Crates -->

[crossbeam-badge]: https://badge-cache.kominick.com/crates/v/crossbeam.svg?label=crossbeam
[crossbeam]: https://docs.rs/crossbeam/
[rayon-badge]: https://badge-cache.kominick.com/crates/v/rayon.svg?label=rayon
[rayon]: https://docs.rs/rayon/

<!-- Reference -->

[`crossbeam::scope`]: https://docs.rs/crossbeam/0.*/crossbeam/fn.scope.html
[`Scope::spawn`]: https://docs.rs/crossbeam/0.*/crossbeam/struct.Scope.html#method.spawn
