# Rayon
[![rayon-badge]][rayon]

## Mutate the elements of an array using parallel processing

```rust

/**
 * An example of mutating array elements in parallel using rayon
 *
 */

extern crate rayon;
use rayon::prelude::*;

fn main() {
    let arr = &mut [0, 7, 9, 11];

    arr.par_iter_mut().for_each(|p| *p -= 1);

    println!("{:?}", arr);
}

```

The example uses the Rayon crate, which is the data parallelism library for Rust. It defines the trait, ParallelIterator, which provides the par_iter_mut() method. This method is actually a call to get a parallel iterator. It lets us write iterator-like chains that execute in parallel. It constructs the values we want and for_each() method, i.e.,the 'operations' method actually carries out the execution.


<!-- Links -->

[rayon-badge]: https://img.shields.io/crates/v/rayon.svg?label=rayon
[rayon]: https://docs.rs/rayon