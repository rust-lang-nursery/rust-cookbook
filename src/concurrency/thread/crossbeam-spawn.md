[ex-crossbeam-spawn]: #ex-crossbeam-spawn
<a name="ex-crossbeam-spawn"></a>
## Spawn a short-lived thread

[![crossbeam-badge]][crossbeam] [![cat-concurrency-badge]][cat-concurrency]

The example uses the [crossbeam] crate, which provides data structures and functions
for concurrent and parallel programming. [`Scope::spawn`] spawns a new scoped thread that is guaranteed
to terminate before returning from the closure that passed into [`crossbeam::scope`] function, meaning that
you can reference data from the calling function.

```rust
extern crate crossbeam;

use std::cmp;

fn main() {
    let arr = &[-4, 1, 10, 25];
    let max = find_max(arr, 0, arr.len());
    assert_eq!(25, max);
}

fn find_max(arr: &[i32], start: usize, end: usize) -> i32 {
    // Perform sequential computation if there are only a few elements.
    const THRESHOLD: usize = 2;
    if end - start <= THRESHOLD {
        return *arr.iter().max().unwrap();
    }

    let mid = start + (end - start) / 2;
    crossbeam::scope(|scope| {
        let left = scope.spawn(|| find_max(arr, start, mid));
        let right = scope.spawn(|| find_max(arr, mid, end));

        cmp::max(left.join(), right.join())
    })
}
```

[`crossbeam::scope`]: https://docs.rs/crossbeam/*/crossbeam/fn.scope.html
[`Scope::spawn`]: https://docs.rs/crossbeam/*/crossbeam/struct.Scope.html#method.spawn
