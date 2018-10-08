## Vector Norm
[![ndarray-badge]][ndarray]

This recipe demonstrates use of the [`Array1`] type, [`dot`], [`map`], and
[`scalar_sum`] in computing the [l1] and [l2] norms of a given vector. The l2
norm calculation is the simpler of the two, as it is the square root of the dot
product of a vector with itself, shown in the function `l2_norm`. The l1 norm,
shown in the function `l1_norm` is computed by mapping the elements of the input
vector to their absolute value, and then calling `scalar_sum`, which reduces the
elements via summation.

Note that both `l1_norm` and `l2_norm` take the [`Array1`] type. This recipe
considers vector norms, so the norm functions only need to accept one
dimensional arrays (hence [`Array1`]).

```rust
extern crate ndarray;

use ndarray::{Array, Array1};

fn l1_norm(x: &Array1<f64>) -> f64 {
  x.mapv(|e| e.abs()).scalar_sum()
}

fn l2_norm(x: &Array1<f64>) -> f64 {
  x.dot(x).sqrt()
}

fn normalize(mut x: Array1<f64>) -> Array1<f64> {
  let norm = l2_norm(&x);
  x.mapv_inplace(|e| e/norm);
  x
}

fn main() {
  let x = Array::from_vec(vec![1., 2., 3., 4., 5.]);
  println!("||x||_2 = {}", l2_norm(&x));
  println!("||x||_1 = {}", l1_norm(&x));
  println!("Normalizing x yields {:?}", normalize(x));
}
```

[l1]: http://mathworld.wolfram.com/L1-Norm.html
[l2]: http://mathworld.wolfram.com/L2-Norm.html
[`Array1`]: https://docs.rs/ndarray/0.12.0/ndarray/type.Array1.html
[`dot`]: https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#method.dot
[`map`]: https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#method.map
[`scalar_sum`]: https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#method.scalar_sum
