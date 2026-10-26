## Adding matrices
[![ndarray-badge]][ndarray] [![cat-science-badge]][cat-science]

Creates two 2-D matrices with [`ndarray::arr2`] and sums them element-wise.

Note the sum is computed as `let sum = &a + &b`. The `&` operator is used to avoid consuming `a` and `b`, making them available later for display. A new array is created containing their sum.

```rust
{{#include ../../../../crates/science/linear_algebra/src/bin/add_matrices.rs }}
```

[`ndarray::arr2`]: https://docs.rs/ndarray/*/ndarray/fn.arr2.html
