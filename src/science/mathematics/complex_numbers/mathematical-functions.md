## Mathematical functions

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
