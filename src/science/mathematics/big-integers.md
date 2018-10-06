# Miscellaneous

## Big integers

[![num-badge]][num] [![cat-science-badge]][cat-science]

If we need to make calculations on integers exceeding size of standard integer type, we can use BigInt.

```rust
extern crate num;

use num::bigint::{BigInt, ToBigInt};

fn factorial(x: i32) -> BigInt {
    let mut factorial: BigInt = 1.to_bigint().unwrap();

    for i in 1..(x+1) {
        factorial = factorial * i;
    }

    factorial
}

fn main() {
    let x = 100;
    println!("{}! equals {}", x, factorial(100));
}
```

{{#include ../../links.md}}