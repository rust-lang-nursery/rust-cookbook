## Avoid discarding errors during error conversions

[![thiserror-badge]][thiserror] [![cat-rust-patterns-badge]][cat-rust-patterns]

The  [thiserror] crate makes [matching] on different error types returned by
a function possible and relatively compact. [`ErrorKind`] determines the error
type.

Uses [reqwest]::[blocking] to query a random integer generator web service.  Converts
the string response into an integer. The Rust standard library,
[reqwest], and the web service can all generate errors. Well defined Rust errors
use [`foreign_links`]. An additional [`ErrorKind`] variant for the web service
error uses `errors` block of the `thiserror` derive macro.

```rust,edition2018
extern crate thiserror;
extern crate reqwest;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Random response error: {0}")]
    RandomResponseError(String),
}

type Result<T> = std::result::Result<T, ErrorKind>;

fn parse_response(response: reqwest::blocking::Response) -> Result<u32> {
  let mut body = response.text()?;
  body.pop();
  body
    .parse::<u32>()
    .map_err(|e| ErrorKind::RandomResponseError(body))
}

fn run() -> Result<()> {
  let url =
    format!("https://www.random.org/integers/?num=1&min=0&max=10&col=1&base=10&format=plain");
  let response = reqwest::blocking::get(&url)?;
  let random_value: u32 = parse_response(response)?;
  println!("a random number between 0 and 10: {}", random_value);
  Ok(())
}

fn main() {
  if let Err(error) = run() {
    match error {
      ErrorKind::Io(_) => println!("Standard IO error: {:?}", error),
      ErrorKind::Reqwest(_) => println!("Reqwest error: {:?}", error),
      ErrorKind::ParseIntError(_) => println!("Standard parse int error: {:?}", error),
      ErrorKind::RandomResponseError(_) => println!("User defined error: {:?}", error),
    }
  }
}
```

[`ErrorKind`]: https://docs.rs/thiserror/*/thiserror/
[`foreign_links`]: https://docs.rs/thiserror/*/thiserror/#foreign-links
[blocking]: https://docs.rs/reqwest/*/reqwest/blocking/index.html
[Matching]:https://docs.rs/thiserror/*/thiserror/#matching-errors
