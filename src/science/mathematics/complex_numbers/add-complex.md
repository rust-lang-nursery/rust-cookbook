## Adding complex numbers

[![num-badge]][num] [![cat-science-badge]][cat-science]

Performing mathematical operations on complex numbers is the same as on
built in types: the numbers in question must be of the same type (i.e. floats
or integers).

```rust
extern crate num;

fn main() {
    let complex_num1 = num::complex::Complex::new(10.0, 20.0); // Must use floats
    let complex_num2 = num::complex::Complex::new(3.1, -4.2);

    let sum = complex_num1 + complex_num2;

    println!("Sum: {}", sum);
}
```
