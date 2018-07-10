## Query the GitHub API

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

Queries GitHub [stargazers API v3](https://developer.github.com/v3/activity/starring/#list-stargazers)
with [`reqwest::get`] to get list of all users who have marked a GitHub project with a star. [`reqwest::Response`] is deserialized with [`Response::json`] into `User` objects implementing [`serde::Deserialize`].

```rust,no_run
# #[macro_use]
# extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}
#
# error_chain! {
#     foreign_links {
#         Reqwest(reqwest::Error);
#     }
# }

fn run() -> Result<()> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);
    let mut response = reqwest::get(&request_url)?;

    let users: Vec<User> = response.json()?;
    println!("{:?}", users);
    Ok(())
}
#
# quick_main!(run);
```

[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`reqwest::Response`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html
[`Response::json`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html#method.json
[`serde::Deserialize`]: https://docs.rs/serde/*/serde/trait.Deserialize.html
