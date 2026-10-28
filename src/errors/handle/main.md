## Handle errors correctly in main

[![anyhow-badge]][anyhow] [![thiserror-badge]][thiserror] [![cat-rust-patterns-badge]][cat-rust-patterns]

As recommended in Rust by Example, [`Box`ing errors] is seen as an easy
strategy for getting started.

```rust,edition2024
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

Deriving [`thiserror::Error`] on an enum takes care of the boilerplate code
needed in order to [handle errors in Rust]. `#[from]` on the
`Io(std::io::Error)` and `ParseInt(std::num::ParseIntError)` variants
generates the `From` implementations that let `?` convert a
[`std::io::Error`] or a [`ParseIntError`] into `UptimeError`. The `#[error]`
attributes provide the `Display` text, and the derive implements the
[`Error`] trait.

The below recipe will tell how long the system has been running by
opening the Unix file `/proc/uptime` and parse the content to get the
first number. Returns uptime unless there is an error.

```rust,edition2024
use std::fs::File;
use std::io::Read;
use thiserror::Error;

#[derive(Error, Debug)]
enum UptimeError {
    #[error("could not read uptime: {0}")]
    Io(#[from] std::io::Error),
    #[error("could not parse uptime: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}

fn read_uptime() -> Result<u64, UptimeError> {
    let mut uptime = String::new();
    File::open("/proc/uptime")?.read_to_string(&mut uptime)?;

    Ok(uptime.split('.').next().unwrap_or_default().parse()?)
}

fn main() {
    match read_uptime() {
        Ok(uptime) => println!("uptime: {} seconds", uptime),
        Err(err) => eprintln!("error: {}", err),
    };
}
```

Application authors can compose enums using `anyhow` can import the `Result`
type from the crate to provide auto-`Box`ing behavior

```rust,edition2024,should_panic
use anyhow::Result;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_string = "yellow".to_string();
    let _my_int = my_string.parse::<i32>()?;
    Ok(())
}
```

[`anyhow`]: https://docs.rs/anyhow/latest/anyhow/
[`Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
[`ParseIntError`]: https://doc.rust-lang.org/std/num/struct.ParseIntError.html
[`std::io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
[`thiserror`]: https://docs.rs/thiserror/latest/thiserror/
[`thiserror::Error`]: https://docs.rs/thiserror/*/thiserror/derive.Error.html

[handle errors in Rust]: https://doc.rust-lang.org/book/second-edition/ch09-00-error-handling.html
[`Box`ing errors]: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
[Designing error types in Rust]: https://mmapped.blog/posts/12-rust-error-handling
