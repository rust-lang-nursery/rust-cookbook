## Check number of logical cpu cores

[![num_cpus-badge]][num_cpus] [![cat-hardware-support-badge]][cat-hardware-support]

Shows the number of logical CPU cores in current machine using [`num_cpus::get`].

This differs from [`std::thread::available_parallelism`] above: `num_cpus::get`
reports the total number of logical cores on the machine, whereas
`available_parallelism` reports only the parallelism available to the current
process, accounting for CPU affinity masks and container limits such as Linux
cgroup quotas. `num_cpus::get` also returns a plain `usize` rather than a
[`Result`], so it needs no error handling, and [`num_cpus::get_physical`]
additionally exposes the count of *physical* cores, which the standard library
does not provide.

```rust,edition2021
fn main() {
    println!("Number of logical cores is {}", num_cpus::get());
}
```

[`num_cpus::get`]: https://docs.rs/num_cpus/*/num_cpus/fn.get.html
[`num_cpus::get_physical`]: https://docs.rs/num_cpus/*/num_cpus/fn.get_physical.html
[`std::thread::available_parallelism`]: https://doc.rust-lang.org/std/thread/fn.available_parallelism.html
[`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
