## Look Up Records Within a Time Range

[![std-badge]][std] [![cat-data-structures-badge]][cat-data-structures]

Stores timestamped sensor readings in a [`BTreeMap`], which keeps keys in sorted order using a B-tree structure. [`BTreeMap::range`] locates the start of the range with binary search in O(log n) time and then iterates forward — so the cost scales with the result size, not the total number of entries. A `HashMap` cannot do this at all; it has no ordering to exploit.

```rust,edition2021
use std::collections::BTreeMap;

fn main() {
    let mut readings: BTreeMap<u32, f64> = BTreeMap::new();
    readings.insert(1_000, 20.1);
    readings.insert(2_000, 21.3);
    readings.insert(3_000, 19.8);
    readings.insert(4_000, 22.5);
    readings.insert(5_000, 23.1);

    let window: Vec<(&u32, &f64)> = readings.range(2_000..=4_000).collect();
    println!("Readings from t=2000 to t=4000:");
    for (ts, temp) in &window {
        println!("  t={ts:>5}  {temp:.1}°C");
    }

    assert_eq!(window.len(), 3);
    assert_eq!(*window[0].0, 2_000);
    assert_eq!(*window[2].0, 4_000);
}
```

[`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
[`BTreeMap::range`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.range
