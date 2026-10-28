## Benchmark a closure with `Instant`

[![std-badge]][std] [![cat-time-badge]][cat-time]

Initializes a `weights` vector, calculates half of its total sum into `half_load`,
and uses `benchmark` to measure the performance of `subset_sum` which takes a slice of weights and a target value and finds a subset whose weights sum to the target.

`benchmark` measures the average time required to call a closure a specified number of times using [`Instant::now`] and [`Instant::elapsed`].
It uses [`black_box`] to prevent the compiler from optimizing away the closure's computation since its return value is ignored.

```rust,edition2024
use std::time::{Duration, Instant};

fn subset_sum(weights: &[u64], target: u64) -> Option<u32> {
    for mask in 0..1u32 << weights.len() {
        let sum: u64 = weights
            .iter()
            .enumerate()
            .filter(|(i, _)| mask >> i & 1 == 1)
            .map(|(_, weight)| weight)
            .sum();

        if sum == target {
            return Some(mask);
        }
    }
    None
}

fn benchmark<F, T>(runs: u32, mut f: F) -> Duration
where
    F: FnMut() -> T,
{
    let start = Instant::now();

    for _ in 0..runs {
        std::hint::black_box(f());
    }

    start.elapsed() / runs
}

fn main() {
    let weights: Vec<u64> = (1..=18).map(|kg| kg * 2).collect();
    let half_load = weights.iter().sum::<u64>() / 2;

    for parcels in 12..=weights.len() {
        let mean = benchmark(3, || subset_sum(&weights[..parcels], half_load));
        println!("{:2} parcels: {:>10.2?}", parcels, mean);
    }
}
```

[`Instant::now`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.now
[`Instant::elapsed`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.elapsed
[`black_box`]: https://doc.rust-lang.org/std/hint/fn.black_box.html
