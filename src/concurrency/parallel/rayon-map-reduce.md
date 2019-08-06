## 並列なMap-reduce

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

この例では30才以上のPersonオブジェクトの年齢の平均を計算するために[`rayon::filter`], [`rayon::map`], [`rayon::reduce`]を使います。

[`rayon::filter`]はコレクションの中から与えられた条件を満たす要素を返します。
[`rayon::map`]は全ての要素を処理して新しいイテレーションを作ります。また、[`rayon::reduce`]は与えられた直前の値と現在の要素に機能します。この例では[`rayon::sum`]で見せた結果と同じになっています。

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
