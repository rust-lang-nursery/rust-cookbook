## Avoid discarding errors during error conversions

[![error-chain-badge]][error-chain] [![cat-rust-patterns-badge]][cat-rust-patterns]

The  [error-chain] crate makes [matching] on different error types returned by
a function possible and relatively compact. [`ErrorKind`] determines the error
type.

Uses [reqwest] to query a random integer generator web service.  Converts
the string response into an integer. The Rust standard library,
[reqwest], and the web service can all generate errors. Well defined Rust errors
use [`foreign_links`]. An additional [`ErrorKind`] variant for the web service
error uses `errors` block of the `error_chain!` macro.

```rust,edition2018
use error_chain::error_chain;
<<<<<<< HEAD
=======

use std::io::Read;
>>>>>>> master

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
        ParseIntError(std::num::ParseIntError);
    }
    errors { RandomResponseError(t: String) }
}

async fn parse_response(response: reqwest::Response) -> Result<u32> {
  let mut body = response.text().await?;
  body.pop();
  body
    .parse::<u32>()
    .chain_err(|| ErrorKind::RandomResponseError(body))
}

async fn run() -> Result<()> {
  let url =
    format!("https://www.random.org/integers/?num=1&min=0&max=10&col=1&base=10&format=plain");
  let response = reqwest::get(&url).await?;
  let random_value: u32 = parse_response(response).await?;
  println!("a random number between 0 and 10: {}", random_value);
  Ok(())
}

#[tokio::main]

async fn main() {
  if let Err(error) = run().await {
    match *error.kind() {
      ErrorKind::Io(_) => println!("Standard IO error: {:?}", error),
      ErrorKind::Reqwest(_) => println!("Reqwest error: {:?}", error),
      ErrorKind::ParseIntError(_) => println!("Standard parse int error: {:?}", error),
      ErrorKind::RandomResponseError(_) => println!("User defined error: {:?}", error),
      _ => println!("Other error: {:?}", error),
    }
  }
}
```

[`ErrorKind`]: https://docs.rs/error-chain/*/error_chain/example_generated/enum.ErrorKind.html
[`foreign_links`]: https://docs.rs/error-chain/*/error_chain/#foreign-links

[Matching]:https://docs.rs/error-chain/*/error_chain/#matching-errors
