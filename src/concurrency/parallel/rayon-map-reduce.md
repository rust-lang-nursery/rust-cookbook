## Map-reduce in parallel

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

This example uses [`rayon::filter`], [`rayon::map`], and [`rayon::reduce`]
to calculate the average age of `Person` objects whose age is over 30.

[`rayon::filter`] returns elements from a collection that satisfy the given
predicate.  [`rayon::map`] performs an operation on every element, creating a
new iteration, and [`rayon::reduce`] performs an operation given the previous
reduction and the current element.  Also shows use of [`rayon::sum`],
which has the same result as the reduce operation in this example.

```rust
extern crate rayon;

use rayon::prelude::*;

struct Person {
    age: u32,
}

fn main() {
    let v: Vec<Person> = vec![
        Person { age: 23 },
        Person { age: 19 },
        Person { age: 42 },
        Person { age: 17 },
        Person { age: 17 },
        Person { age: 31 },
        Person { age: 30 },
    ];

    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30 = v.par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .reduce(|| 0, |x, y| x + y);

    let alt_sum_30: u32 = v.par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .sum();

    let avg_over_30 = sum_over_30 as f32 / num_over_30;
    let alt_avg_over_30 = alt_sum_30 as f32/ num_over_30;

    assert!((avg_over_30 - alt_avg_over_30).abs() < std::f32::EPSILON);
    println!("The average age of people older than 30 is {}", avg_over_30);
}
```

[`rayon::filter`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.filter
[`rayon::map`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.map
[`rayon::reduce`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.reduce
[`rayon::sum`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.sum
