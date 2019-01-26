## Big integers

[![num-badge]][num] [![cat-science-badge]][cat-science]

Calculation for integers exceeding 128 bits are possible with [`BigInt`].

```rust
extern crate num;

use num::bigint::{BigInt, ToBigInt};

fn factorial(x: i32) -> BigInt {
    if let Some(mut factorial) = 1.to_bigint() {
        for i in 1..(x+1) {
            factorial = factorial * i;
        }
        factorial
    }
    else {
        panic!("Failed to calculate factorial!");
    }
}

fn main() {
    println!("{}! equals {}", 100, factorial(100));
}
```

[`BigInt`]: https://docs.rs/num/0.2.0/num/struct.BigInt.html