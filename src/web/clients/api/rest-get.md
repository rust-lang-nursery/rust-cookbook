## Query the GitHub API

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

Queries [GitHub stargazers API v3][github-api-stargazers] with [`reqwest::get`]
to get list of all users who have marked a GitHub repository with a star. 
[`reqwest::Response`] is deserialized into `User` objects implementing [`serde::Deserialize`].

The program expects the GitHub personal access token to be specified in the
environment variable `GITHUB_TOKEN`. Request setup includes the [`reqwest::header::USER_AGENT`]
header as required by the [GitHub API][github-api]. The program deserializes
the response body with [`serde_json::from_str`] into a vector of `User` objects and
processing the response into User instances.

```rust,edition2021,no_run
use serde::Deserialize;
use reqwest::Error;
use reqwest::header::USER_AGENT;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);
    
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(request_url)
        .header(USER_AGENT, "rust-web-api-client") // gh api requires a user-agent header
        .send()?;

    let users: Vec<User> = response.json()?;
    println!("{:?}", users);
    Ok(())
}
```

[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`reqwest::Response`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html
[`Response::json`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html#method.json
[`serde::Deserialize`]: https://docs.rs/serde/*/serde/trait.Deserialize.html
