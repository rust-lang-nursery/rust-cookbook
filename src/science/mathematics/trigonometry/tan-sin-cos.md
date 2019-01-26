## Verifying tan is equal to sin divided by cos

[![std-badge]][std] [![cat-science-badge]][cat-science]

Verifies tan(x) is equal to sin(x)/cos(x) for x = 6.

```rust
fn main() {
    let x: f64 = 6.0;

    let a = x.tan();
    let b = x.sin() / x.cos();

    assert_eq!(a, b);
}
```
