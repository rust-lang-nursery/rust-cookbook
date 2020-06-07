## Query the GitHub API

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

Queries GitHub [stargazers API v3](https://developer.github.com/v3/activity/starring/#list-stargazers)
with [`reqwest::get`] to get list of all users who have marked a GitHub project with a star. 
[`reqwest::Response`] is deserialized with [`Response::json`] into `User` objects implementing [`serde::Deserialize`].

[tokio::main] is used to set up the async executor and the process waits for [`reqwet::get`] to complete before
processing the response into User instances.  

```rust,edition2018,no_run
use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    Ok(())
}
```

[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`reqwest::Response`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html
[`Response::json`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html#method.json
[`serde::Deserialize`]: https://docs.rs/serde/*/serde/trait.Deserialize.html
