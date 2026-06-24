## Check available parallelism

[![std-badge]][std] [![cat-hardware-support-badge]][cat-hardware-support]

Queries the amount of parallelism available to the current process with
[`std::thread::available_parallelism`]. This returns the parallelism the program
can *actually* use: it honors CPU affinity masks and container limits such as
Linux cgroup quotas. The result is a [`NonZeroUsize`], so it is guaranteed to be
at least `1`, and it is wrapped in an [`io::Result`] because the value cannot
always be determined. This has been available in the standard library since Rust
1.59, making it a good default for sizing thread pools.

```rust,edition2021
use std::thread::available_parallelism;

fn main() -> std::io::Result<()> {
    let count = available_parallelism()?.get();
    println!("Number of available threads is {}", count);
    Ok(())
}
```

[`std::thread::available_parallelism`]: https://doc.rust-lang.org/std/thread/fn.available_parallelism.html
[`NonZeroUsize`]: https://doc.rust-lang.org/std/num/type.NonZeroUsize.html
[`io::Result`]: https://doc.rust-lang.org/std/io/type.Result.html
