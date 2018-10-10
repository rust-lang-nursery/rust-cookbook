## Multiply a scalar with a vector with a matrix
[![ndarray-badge]][ndarray] [![cat-science-badge]][cat-science]

Creates a vector with [`ndarray::arr1`] and a matrix with [`ndarray::arr2`]. First, a scalar is multiplied by the vector to get another vector. Then, convert the vector to a column vector with [`ndarray::ArrayBase::reversed_axes`] and multiply the matrix by the column vector to calculate a new matrix.

```rust
extern crate ndarray;

use ndarray::{arr1, arr2, Array1};

fn main() {
    let scalar = 4;

    let vector = arr1(&[1, 2, 3]);

    let matrix = arr2(&[[4, 5, 6],
                        [7, 8, 9]]);

    let new_vector: Array1<_> = scalar * vector;
    println!("{}", new_vector);

    let new_matrix = matrix * new_vector.reversed_axes();
    println!("{}", new_matrix);
}
```

[`ndarray::arr1`]: https://docs.rs/ndarray/*/ndarray/fn.arr1.html
[`ndarray::arr2`]: https://docs.rs/ndarray/*/ndarray/fn.arr2.html
[`ndarray::ArrayBase::reversed_axes`]: https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#method.reversed_axes
