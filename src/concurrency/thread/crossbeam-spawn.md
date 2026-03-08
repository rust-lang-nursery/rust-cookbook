## Spawn a short-lived thread

[![std-badge]][std] [![cat-concurrency-badge]][cat-concurrency]

[`std::thread::scope`] spawns a new scoped thread that is guaranteed
to terminate before returning from the closure, meaning that
you can reference data from the calling function without needing `Arc` or other ownership tricks.

This example splits the array in half and performs the work in separate threads.

```rust,edition2021
fn main() {
    let arr = &[1, 25, -4, 10];
    let max = find_max(arr);
    assert_eq!(max, Some(25));
}

fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;

    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    std::thread::scope(|s| {
        let thread_l = s.spawn(|| find_max(left));
        let thread_r = s.spawn(|| find_max(right));

        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;

        Some(max_l.max(max_r))
    })
}
```

[`std::thread::scope`]: https://doc.rust-lang.org/std/thread/fn.scope.html
