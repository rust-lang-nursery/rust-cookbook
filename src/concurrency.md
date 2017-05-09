# Concurrency

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Mutate the elements of an array in parallel][ex-rayon-iter-mut] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |

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

<!-- Categories -->

[cat-concurrency-badge]: https://img.shields.io/badge/-concurrency-red.svg
[cat-concurrency]: https://crates.io/categories/concurrency

<!-- Crates -->

[rayon-badge]: https://img.shields.io/crates/v/rayon.svg?label=rayon
[rayon]: https://docs.rs/rayon/
