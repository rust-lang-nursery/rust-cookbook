## Multiply a scalar with a vector with a matrix
[![ndarray-badge]][ndarray] [![cat-science-badge]][cat-science]

Creates a 1-D array (vector) with [`ndarray::arr1`] and a 2-D array (matrix)
with [`ndarray::arr2`]. First, a scalar is multiplied by the vector to get
another vector. Then, the matrix is multiplied by the new vector with
[`ndarray::Array2::dot`]. (`dot` performs matrix multiplication, while the `*`
operator performs element-wise multiplication.) In `ndarray`, 1-D arrays can be
interpreted as either row or column vectors depending on context. If
representing the orientation of a vector is important, a 2-D array with one row
or one column must be used instead. In this example, the vector is a 1-D array
on the right-hand side, so `dot` handles it as a column vector.

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

    let new_matrix = matrix.dot(&new_vector);
    println!("{}", new_matrix);
}
```

[`ndarray::Array2::dot`]: https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#method.dot-1
[`ndarray::arr1`]: https://docs.rs/ndarray/*/ndarray/fn.arr1.html
[`ndarray::arr2`]: https://docs.rs/ndarray/*/ndarray/fn.arr2.html
