## Count Word Frequency in a Document

[![std-badge]][std] [![cat-data-structures-badge]][cat-data-structures]

Splits a string into words with [`split_whitespace`] and counts each word's occurrences. [`HashMap::entry`] with [`or_insert`] initializes a missing key to zero and returns a mutable reference, so the increment requires no separate lookup. [`sort_by_key`] with [`Reverse`] sorts the results descending by count; wrapping the count in `Reverse` flips the natural ascending order without a manual comparator.

```rust,edition2021
use std::cmp::Reverse;
use std::collections::HashMap;

fn word_count(text: &str) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let counts = word_count(text);

    let mut pairs: Vec<(&str, usize)> = counts.iter().map(|(&w, &c)| (w, c)).collect();
    pairs.sort_by_key(|&(word, count)| (Reverse(count), word));
    for (word, count) in &pairs {
        println!("{word:>12}  {count}");
    }

    assert_eq!(counts["the"], 3);
    assert_eq!(counts["fox"], 2);
    assert_eq!(counts["quick"], 1);
}
```

[`split_whitespace`]: https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace
[`HashMap::entry`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry
[`or_insert`]: https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert
[`sort_by_key`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_key
[`Reverse`]: https://doc.rust-lang.org/std/cmp/struct.Reverse.html
