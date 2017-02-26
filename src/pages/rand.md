<style TYPE="text/css">
code.has-jax {font: inherit; font-size: 100%; background: inherit; border: inherit;}
</style>
<script type="text/x-mathjax-config">
MathJax.Hub.Config({
    tex2jax: {
        inlineMath: [['$','$'], ['\\(','\\)']],
        skipTags: ['script', 'noscript', 'style', 'textarea', 'pre'] // removed 'code' entry
    }
});
MathJax.Hub.Queue(function() {
    var all = MathJax.Hub.getAllJax(), i;
    for(i = 0; i < all.length; i += 1) {
        all[i].SourceElement().parentNode.className += ' has-jax';
    }
});
</script>
<script type="text/javascript" src="http://cdn.mathjax.org/mathjax/latest/MathJax.js?config=TeX-AMS-MML_HTMLorMML"></script>

# rand
## Random number generators and other randomness functionality
[![rand-badge]][rand]

### Example: Monte carlo integration
Use the `rand` crate to generate random samples and approximate
$\int_{0}^{\pi} sin(x) dx$ using monte carlo.

*Key concepts:*
* Creating thread-specific RNG
* Generating real numbers over an interval
```rust,ignore
extern crate rand;

use rand::Rng;
use std::f32;

/// f(x) = sin(x)
fn f(x: f32) -> f32 {
    x.sin()
}

/// Compute integral of f(x) dx from a to b using n samples
fn monte_carlo(a: f32, b: f32, n: u32) -> f32 {
    // Generate numbers specific to this thread
    let mut rng = rand::thread_rng();

    let mut samples: Vec<f32> = Vec::new();

    // Generate n samples between [a, b)
    for _ in 0..n {
        samples.push(rng.gen_range(a, b)); 
    }

    // Find function values
    let mut sum = 0.;
    for x in samples {
        sum += f(x);
    }
    
    // Returns average of samples over interval
    (b - a) / n as f32 * sum
}

fn main() {
    println!("{}", monte_carlo(0., f32::consts::PI, 200_000));
}
```

### Example: Generating random RGB colors
A *trait* is a language feature that tells the Rust compiler about functionality a type must provide.

Rust has the powerful ability to create traits for your own types.
One example is `rand::Rand`. Any type that implements Rand can use the
polymorphic function `Rng::gen()` to generate random types. 

*Key concepts:*
* Generating a random structure

```rust,ignore
extern crate rand;

use rand::Rng;
use rand::Rand;

#[derive(Debug)] // Allows us to print using {:?} format specifier
struct Color { // RGB Color struct
    r: f64,
    g: f64,
    b: f64,
}

// Implementing Rand for type Color
impl Rand for Color {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        Color {r: rng.next_f64(), b: rng.next_f64(), g: rng.next_f64()}
    }
}

fn main() {
    // Generate a random Color and print to stdout
    let mut rng = rand::thread_rng();
    let c: Color = rng.gen();
    println!("{:?}", c);
}
```


<!-- Links -->

[rand-badge]: https://img.shields.io/crates/v/rand.svg?label=rand
[rand]: https://doc.rust-lang.org/rand/rand/index.html
