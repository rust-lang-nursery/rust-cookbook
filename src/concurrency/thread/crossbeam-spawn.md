## Spawn a short-lived thread

[![crossbeam-badge]][crossbeam] [![cat-concurrency-badge]][cat-concurrency]

The example uses the [crossbeam] crate, which provides data structures and functions
for concurrent and parallel programming. [`Scope::spawn`] spawns a new scoped thread that is guaranteed
to terminate before returning from the closure that passed into [`crossbeam::scope`] function, meaning that
you can reference data from the calling function.

This example splits the array in half and performs the work in separate threads.

```rust
extern crate crossbeam;

fn main() {
    let arr = &[-4, 1, 10, 25];
    let max = find_max(arr, 0, arr.len());
    assert_eq!(max, Some(25));
}

fn find_max(arr: &[i32], start: usize, end: usize) -> Option<i32> {
    const THRESHOLD: usize = 2;
    if end - start <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = start + (end - start) / 2;
    crossbeam::scope(|s| {
        let left = s.spawn(|_| find_max(arr, start, mid));
        let right = s.spawn(|_| find_max(arr, mid, end));
  
        let left = left.join().unwrap()?;
        let right = right.join().unwrap()?;
  
        Some(left.max(right))
    }).unwrap()
}
```

[`crossbeam::scope`]: https://docs.rs/crossbeam/*/crossbeam/fn.scope.html
[`Scope::spawn`]: https://docs.rs/crossbeam/*/crossbeam/thread/struct.Scope.html#method.spawn
