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
$\int_{0}^{\pi} sin(x) dx$ using monte carlo method.
```rust
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

<!-- Links -->
[rand-badge]: https://img.shields.io/crates/v/rand.svg?label=rand
[rand]: https://doc.rust-lang.org/rand/rand/index.html
