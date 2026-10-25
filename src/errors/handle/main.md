## Handle errors correctly in main

[![anyhow-badge]][anyhow] [![cat-rust-patterns-badge]][cat-rust-patterns]



# Error Strategies (2024)

As recommended in Rust by Example, [`Box`ing errors] is seen as an easy
strategy for getting started.

```rust,edition2018
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Example of boxing errors
    let result: Result<(), Box<dyn Error>> = Ok(());
    result
}
```

To understand what kind of error handling may be required study [Designing
error types in Rust] and consider [`thiserror`] for libraries or [`anyhow`] as
a maintained error aggregation option.

```rust,edition2018
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MultiError {
    #[error("🦀 got {0}")]
    ErrorClass(String),
}

fn main() -> Result<(), MultiError> {
    // Example of using thiserror
    Ok(())
}
```

Application authors can compose enums using `anyhow` can import the `Result`
type from the crate to provide auto-`Box`ing behavior

```rust,edition2018,should_panic
use anyhow::Result;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_string = "yellow".to_string();
    let _my_int = my_string.parse::<i32>()?;
    Ok(())
}
```

## Handle errors with anyhow

The [`anyhow`] library simplifies error handling in application code. This example reads the system uptime from `/proc/uptime` and adds context to errors that occur while reading or parsing the file.

```rust,edition2018

use anyhow::{Context, Result};
use std::fs;

fn read_uptime() -> Result<u64> {
    let contents = fs::read_to_string("/proc/uptime")
        .context("Failed to read /proc/uptime")?;

    let uptime = contents
        .split('.')
        .next()
        .context("Cannot parse uptime data")?
        .parse()
        .context("Failed to parse uptime")?;

    Ok(uptime)
}

fn main() -> Result<()> {
    let uptime = read_uptime()?;
    println!("uptime: {} seconds", uptime);
    Ok(())
}
```

[`anyhow`]: https://docs.rs/anyhow/latest/anyhow/
[`Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
[`std::io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
[`thiserror`]: https://docs.rs/thiserror/latest/thiserror/
[handle errors in Rust]: https://doc.rust-lang.org/book/second-edition/ch09-00-error-handling.html
[`Box`ing errors]: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
[Designing error types in Rust]: https://mmapped.blog/posts/12-rust-error-handling
