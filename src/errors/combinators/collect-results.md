## Collect an iterator of Results

[![std-badge]][std] [![cat-rust-patterns-badge]][cat-rust-patterns]

When every item of an iterator is fallible, [`Iterator::collect`] can gather
them into a single `Result<Vec<_>, _>`. It returns the whole vector only if
every element is `Ok`, short-circuiting to the first `Err` otherwise. When
failures should instead be skipped, [`Iterator::filter_map`] paired with
[`Result::ok`] keeps only the successful values.

[`Iterator::collect`] needs to know which collection to build. The two calls
below show the interchangeable ways to tell it: a turbofish on `collect`
(`collect::<Result<Vec<_>, _>>()`) or a type annotation on the binding
(`let result: Result<Vec<i32>, _>`). Pick whichever reads better at the call
site; the result is identical.

```rust,edition2021
fn main() -> Result<(), std::num::ParseIntError> {
    let raw = ["1", "2", "3"];

    // Collecting into `Result<Vec<_>, _>` yields the whole vector only
    // when every item parses successfully.
    let numbers: Vec<i32> = raw
        .iter()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;
    println!("all parsed:    {numbers:?}");
    assert_eq!(numbers, vec![1, 2, 3]);

    // A single bad value short-circuits the entire collection into `Err`.
    let mixed = ["1", "x", "3"];
    let result: Result<Vec<i32>, _> = mixed.iter().map(|s| s.parse::<i32>()).collect();
    println!("short-circuit: {result:?}");
    assert!(result.is_err());

    // To ignore failures instead, use `filter_map` to keep the `Ok` values.
    let valid: Vec<i32> = mixed.iter().filter_map(|s| s.parse::<i32>().ok()).collect();
    println!("skip failures: {valid:?}");
    assert_eq!(valid, vec![1, 3]);

    Ok(())
}
```

[`Result::ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.ok
