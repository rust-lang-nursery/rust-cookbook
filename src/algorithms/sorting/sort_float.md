## Sort a Vector of Floats

[![std-badge]][std] [![cat-science-badge]][cat-science]

A Vector of f32 or f64 can be sorted with [`vec::sort_by`] and [`f64::total_cmp`].
Unlike [`PartialOrd::partial_cmp`], [`total_cmp`] handles NaN values without panicking
by placing them at the end of the sort order.

```rust,edition2021
fn main() {
    let mut vec = vec![1.1_f64, 1.15, 5.5, 1.123, 2.0];

    vec.sort_by(|a, b| a.total_cmp(b));

    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
}
```

[`vec::sort_by`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by
[`f64::total_cmp`]: https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp
[`total_cmp`]: https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp
[`PartialOrd::partial_cmp`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#tymethod.partial_cmp
