## Check number of logical cpu cores

[![num_cpus-badge]][num_cpus] [![cat-hardware-support-badge]][cat-hardware-support]

Shows the number of logical CPU cores in current machine using [`num_cpus::get`].

```rust,edition2018
fn main() {
    println!("Number of logical cores is {}", num_cpus::get());
}
```
