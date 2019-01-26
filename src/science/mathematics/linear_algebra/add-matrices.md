## Adding matrices
[![ndarray-badge]][ndarray] [![cat-science-badge]][cat-science]

Creates two matrices with [`ndarray::arr2`] and adds them together.

```rust
extern crate ndarray;

use ndarray::arr2;

fn main() {
    let a = arr2(&[[1, 2, 3],
                   [4, 5, 6]]);

    let b = arr2(&[[6, 5, 4],
                   [3, 2, 1]]);

    println!("Sum: {}", a + b);
}
```

[`ndarray::arr2`]: https://docs.rs/ndarray/*/ndarray/fn.arr2.html
