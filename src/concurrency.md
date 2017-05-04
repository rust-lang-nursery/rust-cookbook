# Concurrency

| Recipe | Crates |
|--------|--------|
| [Mutate the elements of an array in parallel][ex-rayon-iter-mut] | [![rayon-badge]][rayon] |

[ex-rayon-iter-mut]: #ex-rayon-iter-mut
<a name="ex-rayon-iter-mut"></a>
## Mutate the elements of an array in parallel

[![rayon-badge]][rayon]

```rust
extern crate rayon;
use rayon::prelude::*;

fn main() {
    let arr = &mut [0, 7, 9, 11];

    arr.par_iter_mut().for_each(|p| *p -= 1);

    println!("{:?}", arr);
}
```

The example uses the Rayon crate, which is a data parallelism library for Rust.
It defines the trait `ParallelIterator` which provides the `par_iter_mut()`
method. This method is a call to get a parallel iterator. It lets us write
iterator-like chains that execute in parallel. It constructs the values we want
and finally the `for_each()` method which actually carries out the execution.

<!-- Crates -->

[rayon-badge]: https://img.shields.io/crates/v/rayon.svg?label=rayon
[rayon]: https://docs.rs/rayon/
