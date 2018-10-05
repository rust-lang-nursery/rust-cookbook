## Complex numbers

### Creating complex numbers

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

### Adding complex numbers

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

### Mathematical functions

[![num-badge]][num] [![cat-science-badge]][cat-science]

Complex numbers have a range of interesting properties when it comes to
how they interact with other mathematical functions, most notibly the family
of sine functions as well as the number e. To use these functions with
complex numbers, the Complex type has a few built in
functions, all of which can be found here: [`num::complex::Complex`].

```rust
extern crate num;

use std::f64::consts::PI;
use num::complex::Complex;

fn main() {
    let x = Complex::new(0.0, 2.0*PI);
    
    println!("e^(2i * pi) = {}", x.exp()); // =~1
}
```

[`num::complex::Complex`]: https://autumnai.github.io/cuticula/num/complex/struct.Complex.html

{{#include ../../links.md}}
