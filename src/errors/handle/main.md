## Handle errors correctly in main

[![anyhow-badge]][anyhow] [![cat-rust-patterns-badge]][cat-rust-patterns]
[![thiserror-badge]][thiserror] [![cat-rust-patterns-badge]][cat-rust-patterns]
[![error-chain-badge]][error-chain] [![cat-rust-patterns-badge]][cat-rust-patterns]

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

#[derive(Error,Debug)]
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

# Error Chain (2015-2018)
Handles error that occur when trying to open a file that does not
exist. It is achieved by using [error-chain], a library that takes
care of a lot of boilerplate code needed in order to [handle errors in Rust].

`Io(std::io::Error)` inside [`foreign_links`] allows automatic
conversion from [`std::io::Error`] into [`error_chain!`] defined type
implementing the [`Error`] trait.

The below recipe will tell how long the system has been running by
opening the Unix file `/proc/uptime` and parse the content to get the
first number. Returns uptime unless there is an error.

Other recipes in this book will hide the [error-chain] boilerplate, and can be
seen by expanding the code with the ⤢ button.

```rust,edition2018,ignore
use error_chain::error_chain;

use std::fs::File;
use std::io::Read;

error_chain!{
    foreign_links {
        Io(std::io::Error);
        ParseInt(::std::num::ParseIntError);
    }
}

fn read_uptime() -> Result<u64> {
    let mut uptime = String::new();
    File::open("/proc/uptime")?.read_to_string(&mut uptime)?;

    Ok(uptime
        .split('.')
        .next()
        .ok_or("Cannot parse uptime data")?
        .parse()?)
}

fn main() {
    match read_uptime() {
        Ok(uptime) => println!("uptime: {} seconds", uptime),
        Err(err) => eprintln!("error: {}", err),
    };
}
```

[`anyhow`]: https://docs.rs/anyhow/latest/anyhow/
[`error_chain!`]: https://docs.rs/error-chain/*/error_chain/macro.error_chain.html
[`Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
[`foreign_links`]: https://docs.rs/error-chain/*/error_chain/#foreign-links
[`std::io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
[`thiserror`]: https://docs.rs/thiserror/latest/thiserror/

[handle errors in Rust]: https://doc.rust-lang.org/book/second-edition/ch09-00-error-handling.html
[`Box`ing errors]: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
[Designing error types in Rust]: https://mmapped.blog/posts/12-rust-error-handling
