### Measures of central tendency

[![std-badge]][std]

Measures of central tendency summarize a set of data as an "average" or "expected" value that is likely to be representative of a typical member of the data set in some way. The most common measures of central tendency are the
- mean (the sum of all measurements divided by the number of measurements in the set)
- median (the value in the middle of a sorted set, or mean of the two middle values when the size of the set is even), and
- mode (element of the set that appears with the highest frequency)

This example calculates these measures for a data set contained within a Rust array, passed to functions as a [`slice`]. Any collection that can produce an Iterator, such as a Vector, may use a similar approach.

The example calculates the mean by producing an iterator of references over the data, and using [`sum`] to determine the total value. The slice itself contains the number of elements, obtained via [`len`]. A simple division provides the mean, with conversions to floating point types to account for rational components of the result.

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

Calculating the median [`sort`]s the data in-place, requiring the input slice to be mutable. If the number of elements is even, calculate the mean of the middle two measurements, which can be passed to the previous mean function as a sub[`slice`] of the original slice. For odd-sized data sets, simply pick the measurement from the middle of the sorted set.

```rust
fn main() {
    let mut data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    data.sort();

    let count = data.len();

    let median = match count {
        odd if odd % 2 != 0 => Some(data[odd / 2] as f32),
        even if even > 0 => {
            let fst_middle = data[(even / 2) - 1];
            let snd_middle = data[(even / 2)];

            Some((fst_middle + snd_middle) as f32 / 2.0)
        },
        _ => None
    };

    println!("Median of the data is {:?}", median);
}
```

The mode uses a mutable [`HashMap`] to collect counts of each distinct integer from the set, using a [`fold`]. Each value is accumulated into the [`HashMap`] using the [`entry`] API. Once these counts have been collected, the largest count is selected via the iterator function [`max_by_key`]. [`map`] then retrieves and dereferences the value within the wrapping [`Option`].

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

As the data may be empty in the input of any of the three measures, there may be no mean, median or mode to calculate. Each function therefore returns an [`Option`], to be handled by the caller.

[sum]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum
[len]: https://doc.rust-lang.org/std/primitive.slice.html#method.len
[sort]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort
[slice]: https://doc.rust-lang.org/std/primitive.slice.html
[HashMap]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[fold]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
[entry]: https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
[max_by_key]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by_key
[map]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map
[Option]: https://doc.rust-lang.org/std/option/enum.Option.html