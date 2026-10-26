## Vector comparison
[![ndarray-badge]][ndarray]

The [ndarray] crate supports a number of ways to create arrays -- this recipe creates 
[`ndarray::Array`]s from `std::Vec` using `from`. Then, it sums the arrays element-wise.

This recipe compares two floating-point vectors element-wise. 
For simple cases, we can use `assert_eq!` for exact equality comparison. For more 
complex floating-point comparisons that need to handle precision issues, you can use 
the [`approx`] crate. Enabling the `approx` feature in the `ndarray` dependency
(for example, `ndarray = { version = "0.17", features = ["approx"] }`) lets whole
arrays be compared directly. This recipe ends with [`assert_abs_diff_eq!`], which
passes when two sequences differ by less than the given `epsilon`. It compares the
arrays' data as slices, which only needs the `approx` crate itself.

This recipe also demonstrates additional ownership behavior. Here, `let z = a + b` consumes 
`a` and `b`, updates `a` with the result, then moves ownership to `z`. Alternatively, 
`let w = &c + &d` creates a new vector without consuming `c` or `d`, allowing 
their modification later. See [Binary Operators With Two Arrays] for additional detail.

```rust
{{#include ../../../../crates/science/linear_algebra/src/bin/vector_comparison.rs }}
```

[`approx`]: https://docs.rs/approx/*/approx/index.html
[`assert_abs_diff_eq!`]: https://docs.rs/approx/*/approx/macro.assert_abs_diff_eq.html
[Binary Operators With Two Arrays]: https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#binary-operators-with-two-arrays
[ndarray]: https://docs.rs/crate/ndarray/*
[`ndarray::Array`]: https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html
