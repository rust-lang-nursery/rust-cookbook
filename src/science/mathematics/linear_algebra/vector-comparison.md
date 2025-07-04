## Vector comparison
[![ndarray-badge]][ndarray]

The [ndarray] crate supports a number of ways to create arrays -- this recipe creates 
[`ndarray::Array`]s from `std::Vec` using `from`. Then, it sums the arrays element-wise.

This recipe contains an example of comparing two floating-point vectors element-wise. 
For simple cases, we can use `assert_eq!` for exact equality comparison. For more 
complex floating-point comparisons that need to handle precision issues, you can use 
the [`approx`] crate with the `approx` feature enabled in the `ndarray` dependency 
in `Cargo.toml`. For example, `ndarray = { version = "0.13", features = ["approx"] }`.

This recipe also contains additional ownership examples. Here, `let z = a + b` consumes 
`a` and `b`, updates `a` with the result, then moves ownership to `z`. Alternatively, 
`let w = &c + &d` creates a new vector without consuming `c` or `d`, allowing 
their modification later. See [Binary Operators With Two Arrays] for additional detail.

```rust,edition2018
extern crate ndarray;
use ndarray::Array;

fn main() {
  let a = Array::from(vec![1., 2., 3., 4., 5.]);
  let b = Array::from(vec![5., 4., 3., 2., 1.]);
  let mut c = Array::from(vec![1., 2., 3., 4., 5.]);
  let mut d = Array::from(vec![5., 4., 3., 2., 1.]);

  let z = a + b;
  let w =  &c + &d;

  assert_eq!(z, Array::from(vec![6., 6., 6., 6., 6.]));

  println!("c = {}", c);
  c[0] = 10.;
  d[1] = 10.;

  assert_eq!(w, Array::from(vec![6., 6., 6., 6., 6.]));

}
```

[`approx`]: https://docs.rs/approx/*/approx/index.html
[`approx`]: https://docs.rs/approx/*/approx/index.html
[Binary Operators With Two Arrays]: https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#binary-operators-with-two-arrays
[ndarray]: https://docs.rs/crate/ndarray/*
[`ndarray::Array`]: https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html
