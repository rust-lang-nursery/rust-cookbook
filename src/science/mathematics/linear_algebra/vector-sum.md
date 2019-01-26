## Vector Sum
[![ndarray-badge]][ndarray]

The [ndarray] crate supports a number of ways to create arrays -- this recipe
focuses on creating [`ndarray::Array`]s from `std::Vec` via [`from_vec`]. Adding two
arrays together is no different than adding two numbers together. Using the `&`
operand on the arrays within an arithmetic operation prevents the operation from
consuming the arrays. Without `&`, the arrays are consumed.

In the first example, arrays `a` and `b` are moved in the let-statement `z = a +
b`. In the second example, the arrays `c` and `d` are not moved and instead, a
new array is created for `w`. Updating either of `c` or `d` after the vector sum
has no effect the value of `w`. Additionally, while printing `c` works as
expected, it would be an error to print `b` due to the move. See [Binary
Operators With Two Arrays] for additional detail.

```rust
extern crate ndarray;
use ndarray::Array;

fn main() {
  let a = Array::from_vec(vec![1., 2., 3., 4., 5.]);
  let b = Array::from_vec(vec![5., 4., 3., 2., 1.]);
  let mut c = Array::from_vec(vec![1., 2., 3., 4., 5.]);
  let mut d = Array::from_vec(vec![5., 4., 3., 2., 1.]);

  let z = a + b;
  let w =  &c + &d;

  let epsilon = 1e-8;
  for elem in z.iter() {
    let diff: f32 = *elem - 6.;
    assert!(diff.abs() < epsilon);
  }

  println!("c = {}", c);
  c[0] = 10.;
  d[1] = 10.;

  for elem in w.iter() {
    let diff: f32 = *elem - 6.;
    assert!(diff.abs() < epsilon);
  }

}
```

[Binary Operators With Two Arrays]: https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#binary-operators-with-two-arrays
[`from_vec`]: https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#method.from_vec
[ndarray]: https://docs.rs/crate/ndarray/*
[`ndarray::Array`]: https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html
