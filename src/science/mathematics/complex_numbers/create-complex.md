## Creating complex numbers

[![num-badge]][num] [![cat-science-badge]][cat-science]

Creates complex numbers of type [`num::complex::Complex`]. Both the real and
imaginary part of the complex number must be of the same type.

```rust
extern crate num;

fn main() {
    let complex_integer = num::complex::Complex::new(10, 20);
    let complex_float = num::complex::Complex::new(10.1, 20.1);

    println!("Complex integer: {}", complex_integer);
    println!("Complex float: {}", complex_float);
}
```

[`num::complex::Complex`]: https://autumnai.github.io/cuticula/num/complex/struct.Complex.html
