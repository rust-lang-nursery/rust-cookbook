## Basic Authentication

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

Uses [`reqwest::RequestBuilder::basic_auth`] to perform a basic HTTP authentication.

```rust,edition2021,no_run
use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {
    let client = Client::new();

    let user_name = "testuser".to_string();
    let password: Option<String> = None;

    let response = client
        .get("https://httpbin.org/")
        .basic_auth(user_name, password)
        .send();

    println!("{:?}", response);

    Ok(())
}
```

[`reqwest::RequestBuilder::basic_auth`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.basic_auth
