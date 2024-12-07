## Consume a paginated RESTful API

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

Wraps a paginated web API in a convenient Rust iterator. The iterator lazily
fetches the next page of results from the remote server as it arrives at the end of each page.

This file is named paginated.rs.
```rust,edition2024,no_run
{{#include ../../../../crates/web/src/paginated.rs}}
```

In this main we import the paginated file and use the `ReverseDependencies` iterator to fetch all the crates that depend on the `serde` crate.  This file is named main.rs.  The two files are in the same directory.

```rust,no_run
mod paginated;

fn main() -> Result<()> {
    for dep in paginated::ReverseDependencies::of("serde")? {
        let dependency = dep?;
        println!("{} depends on {}", dependency.id, dependency.crate_id);
    }
    Ok(())
}
```
