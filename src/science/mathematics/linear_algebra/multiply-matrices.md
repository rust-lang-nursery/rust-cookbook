## Multiplying matrices
[![ndarray-badge]][ndarray] [![cat-science-badge]][cat-science]

Creates two matrices with [`ndarray::arr2`] and performs matrix multiplication on them with [`ndarray::ArrayBase::dot`].

```rust
{{#include ../../../../crates/science/linear_algebra/src/bin/multiply_matrices.rs }}
```

[`ndarray::arr2`]: https://docs.rs/ndarray/*/ndarray/fn.arr2.html
[`ndarray::ArrayBase::dot`]: https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#method.dot-1
<!-- needs-no-features: ndarray -->
