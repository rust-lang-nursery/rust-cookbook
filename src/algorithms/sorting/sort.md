## 整数のベクタをソートする

[![std-badge]][std] [![cat-science-badge]][cat-science]

このサンプルでは[`vec::sort`]を使って整数の整数のベクタをソートしています。代わりに[`vec::sort_unstable`] を使用することもできます。高速になりますが等しい要素の順序は保持されません。

```rust
fn main() {
    let mut vec = vec![1, 5, 10, 2, 15];
    
    vec.sort();

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}
```

[`vec::sort`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort
[`vec::sort_unstable`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_unstable
