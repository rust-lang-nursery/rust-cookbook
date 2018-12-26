### Measures of central tendency

[![std-badge]][std] [![cat-science-badge]][cat-science]

These examples calculate measures of central tendency for a data set contained within a Rust array. There may be no mean, median or mode to calculate for an empty set of data, so each function returns an [`Option`] to be handled by the caller.

The first example calculates the mean (the sum of all measurements divided by the number of measurements in the set) by producing an iterator of references over the data, and using [`sum`] and [`len`] to determine the total value and count of values respectively.

```rust
fn main() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let sum = data.iter().sum::<i32>() as f32;
    let count = data.len();

    let mean = match count {
       positive if positive > 0 => Some(sum  / count as f32),
       _ => None
    };

    println!("Mean of the data is {:?}", mean);
}
```

The second example calculates the median using the quickselect algorithm, which avoids a full [`sort`] by sorting only partitions of the data set known to possibly contain the median. This uses [`cmp`] and [`Ordering`] to succinctly decide the next partition to examine, and [`split_at`] to choose an arbitrary pivot for the next partition at each step.

```rust
use std::cmp::Ordering;

fn partition(data: &[i32]) -> Option<(Vec<i32>, i32, Vec<i32>)> {
    match data.len() {
        0 => None,
        _ => {
            let (pivot_slice, tail) = data.split_at(1);
            let pivot = pivot_slice[0];
            let (left, right) = tail.iter()
                .fold((vec![], vec![]), |mut splits, next| {
                    {
                        let (ref mut left, ref mut right) = &mut splits;
                        if next < &pivot {
                            left.push(*next);
                        } else {
                            right.push(*next);
                        }
                    }
                    splits
                });

            Some((left, pivot, right))
        }
    }
}

fn select(data: &[i32], k: usize) -> Option<i32> {
    let part = partition(data);

    match part {
        None => None,
        Some((left, pivot, right)) => {
            let pivot_idx = left.len();

            match pivot_idx.cmp(&k) {
                Ordering::Equal => Some(pivot),
                Ordering::Greater => select(&left, k),
                Ordering::Less => select(&right, k - (pivot_idx + 1)),
            }
        },
    }
}

fn median(data: &[i32]) -> Option<f32> {
    let size = data.len();

    match size {
        even if even % 2 == 0 => {
            let fst_med = select(data, (even / 2) - 1);
            let snd_med = select(data, even / 2);

            match (fst_med, snd_med) {
                (Some(fst), Some(snd)) => Some((fst + snd) as f32 / 2.0),
                _ => None
            }
        },
        odd => select(data, odd / 2).map(|x| x as f32)
    }
}

fn main() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let part = partition(&data);
    println!("Partition is {:?}", part);

    let sel = select(&data, 5);
    println!("Selection at ordered index {} is {:?}", 5, sel);

    let med = median(&data);
    println!("Median is {:?}", med);
}
```

The final example calculates the  mode using a mutable [`HashMap`] to collect counts of each distinct integer from the set, using a [`fold`] and the [`entry`] API. The most frequent value in the [`HashMap`] surfaces with [`max_by_key`].

```rust
use std::collections::HashMap;

fn main() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let frequencies = data.iter().fold(HashMap::new(), |mut freqs, value| {
        *freqs.entry(value).or_insert(0) += 1;
        freqs
    });

    let mode = frequencies
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| *value);

    println!("Mode of the data is {:?}", mode);
}
```

[Option]: https://doc.rust-lang.org/std/option/enum.Option.html
[sum]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum
[len]: https://doc.rust-lang.org/std/primitive.slice.html#method.len
[sort]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort
[cmp]: https://doc.rust-lang.org/beta/std/cmp/trait.Ord.html#tymethod.cmp
[Ordering]: https://doc.rust-lang.org/beta/std/cmp/enum.Ordering.html
[split_at]: https://doc.rust-lang.org/std/primitive.slice.html#method.split_at
[HashMap]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[fold]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
[entry]: https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
[max_by_key]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by_key
