## Set Operations with HashSet

[![std-badge]][std] [![cat-data-structures-badge]][cat-data-structures]

Computes the [`difference`], [`intersection`], and [`union`] of two [`HashSet`]s. Each operation returns an iterator of references; [`copied`] converts `&&str` to `&str` for easier collection.

```rust,edition2021
use std::collections::HashSet;

fn main() {
    let deployed: HashSet<&str> = ["web", "api", "worker", "scheduler"]
        .iter()
        .copied()
        .collect();
    let monitored: HashSet<&str> = ["web", "api", "database"]
        .iter()
        .copied()
        .collect();

    // Services running but not being monitored
    let mut unmonitored: Vec<&str> = deployed.difference(&monitored).copied().collect();
    unmonitored.sort();
    println!("Deployed but not monitored: {unmonitored:?}");
    assert_eq!(unmonitored, ["scheduler", "worker"]);

    // Services present in both sets
    let mut common: Vec<&str> = deployed.intersection(&monitored).copied().collect();
    common.sort();
    println!("Monitored and deployed:     {common:?}");
    assert_eq!(common, ["api", "web"]);

    // All services mentioned across both sets
    let mut all: Vec<&str> = deployed.union(&monitored).copied().collect();
    all.sort();
    println!("All services:               {all:?}");
    assert_eq!(all.len(), 5);
}
```

[`HashSet`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
[`difference`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.difference
[`intersection`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.intersection
[`union`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.union
[`copied`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.copied
