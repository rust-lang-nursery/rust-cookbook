[ex-rand]: #ex-rand
<a name="ex-rand"></a>
## Generate random numbers

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

Generates random numbers with help of random-number
generator [`rand::Rng`] obtained via [`rand::thread_rng`].

```rust
extern crate rand;

use rand::Rng;

fn main() {
    // Each thread has an automatically-initialised random number generator:
    let mut rng = rand::thread_rng();

    // Integers are uniformly distributed over the type's whole range:
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());

    // Floating point numbers are uniformly distributed in the half-open range [0, 1)
    println!("Random float: {}", rng.gen::<f64>());
}
```

[`rand::Rng`]: https://doc.rust-lang.org/rand/0.4/rand/trait.Rng.html
[`rand::thread_rng`]: https://doc.rust-lang.org/rand/0.4/rand/fn.thread_rng.html
