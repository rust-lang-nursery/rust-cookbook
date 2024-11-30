## Query the GitHub API

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

Queries GitHub [stargazers API v3](https://developer.github.com/v3/activity/starring/#list-stargazers)
with [`reqwest::get`] to get list of all users who have marked a GitHub project with a star. 
[`reqwest::Response`] is deserialized with [`Response::json`] into `User` objects implementing [`serde::Deserialize`].

[tokio::main] is used to set up the async executor and the process waits for [`reqwest::get`] to complete before
processing the response into User instances.

to set up the crates required to run this example run

```
cargo new
cargo add reqwest serde tokio
```

edit the Cargo.toml to add features

```
reqwest = { version = "..", features = ["json"] }
serde = { version = "", features = ["derive"] }
tokio = { version = "..", features = ["full"] }
```

```rust,edition2024
use serde::Deserialize;
use reqwest::Error;
use reqwest::header::USER_AGENT;

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
    
    let client = reqwest::Client::new();
    let response = client
        .get(request_url)
        .header(USER_AGENT, "rust-web-api-client") // gh api requires a user-agent header
        .send()
        .await?;

    let users: Vec<User> = response.json().await?;
    users.iter().for_each(|user| println!("{:?} {}({})", user, user.login, user.id));

    Ok(())
}
```

[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`reqwest::Response`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html
[`Response::json`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html#method.json
[`serde::Deserialize`]: https://docs.rs/serde/*/serde/trait.Deserialize.html
