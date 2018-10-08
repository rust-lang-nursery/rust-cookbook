## Vector Norm
[![ndarray-badge]][ndarray]

This recipe demonstrates use of the [`Array1`] type, [`ArrayView1`] type,
[`fold`] method, and [`dot`] method in computing the [l1] and [l2] norms of a
given vector. The l2 norm calculation is the simpler of the two, as it is the
square root of the dot product of a vector with itself, shown in the function
`l2_norm`. The l1 norm, shown in the function `l1_norm`, is computed by a `fold`
operation that sums the absolute values of the elements. (This could also be
performed with `x.mapv(f64::abs).scalar_sum()`, but that would allocate a new
array for the result of the `mapv`.)

Note that both `l1_norm` and `l2_norm` take the [`ArrayView1`] type. This recipe
considers vector norms, so the norm functions only need to accept one
dimensional views (hence [`ArrayView1`]). While the functions could take a
parameter of type `&Array1<f64>` instead, that would require the caller to have
a reference to an owned array, which is more restrictive than just having access
to a view (since a view can be created from any array or view, not just an owned
array). The most convenient argument type for the caller would be
`&ArrayBase<S, Ix1> where S: Data`, because then the caller could use `&array`
or `&view` instead of `x.view()`. If the function is part of your public API,
that may be a better choice for the benefit of your users, but for internal
functions, the more concise `ArrayView1<f64>` may be preferable.

```rust
#[macro_use(array)]
extern crate ndarray;

use ndarray::{Array1, ArrayView1};

fn l1_norm(x: ArrayView1<f64>) -> f64 {
    x.fold(0., |acc, elem| acc + elem.abs())
}

fn l2_norm(x: ArrayView1<f64>) -> f64 {
    x.dot(&x).sqrt()
}

fn normalize(mut x: Array1<f64>) -> Array1<f64> {
    let norm = l2_norm(x.view());
    x.mapv_inplace(|e| e/norm);
    x
}

fn main() {
    let x = array![1., 2., 3., 4., 5.];
    println!("||x||_2 = {}", l2_norm(x.view()));
    println!("||x||_1 = {}", l1_norm(x.view()));
    println!("Normalizing x yields {:?}", normalize(x));
}
```

[l1]: http://mathworld.wolfram.com/L1-Norm.html
[l2]: http://mathworld.wolfram.com/L2-Norm.html
[`Array1`]: https://docs.rs/ndarray/*/ndarray/type.Array1.html
[`ArrayView1`]: https://docs.rs/ndarray/*/ndarray/type.ArrayView1.html
[`dot`]: https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#method.dot
[`fold`]: https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#method.fold
