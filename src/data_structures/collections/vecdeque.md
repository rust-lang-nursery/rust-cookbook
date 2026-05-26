## Sliding-Window Buffer with VecDeque

[![std-badge]][std] [![cat-data-structures-badge]][cat-data-structures]

Maintains a fixed-size window over a stream of values using [`VecDeque`]. [`push_back`] appends new values and [`pop_front`] discards the oldest, keeping memory bounded regardless of input length. [`with_capacity`] is optional — `VecDeque::new()` works fine — but pre-allocating the window size avoids reallocations as the buffer fills up for the first time. [`zip`] pairs each original reading with its computed average for display.

```rust,edition2021
use std::collections::VecDeque;

fn sliding_average(values: &[f64], window: usize) -> Vec<f64> {
    let mut buf: VecDeque<f64> = VecDeque::with_capacity(window);
    let mut averages = Vec::with_capacity(values.len());

    for &v in values {
        if buf.len() == window {
            buf.pop_front();
        }
        buf.push_back(v);
        averages.push(buf.iter().sum::<f64>() / buf.len() as f64);
    }
    averages
}

fn main() {
    let readings = [1.0, 2.0, 3.0, 4.0, 5.0];
    let averages = sliding_average(&readings, 3);

    println!("reading  average");
    for (r, a) in readings.iter().zip(&averages) {
        println!("  {r:.1}      {a:.2}");
    }

    assert_eq!(averages[0], 1.0);  // window: [1.0]
    assert_eq!(averages[1], 1.5);  // window: [1.0, 2.0]
    assert_eq!(averages[2], 2.0);  // window: [1.0, 2.0, 3.0]
    assert_eq!(averages[3], 3.0);  // window: [2.0, 3.0, 4.0]
    assert_eq!(averages[4], 4.0);  // window: [3.0, 4.0, 5.0]
}
```

[`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
[`with_capacity`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.with_capacity
[`push_back`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.push_back
[`pop_front`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.pop_front
[`zip`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip
