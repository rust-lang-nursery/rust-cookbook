use reqwest::header::USER_AGENT;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct User {
    login: String,
    id: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    println!("{}", request_url);

    let token = env::var("GITHUB_TOKEN")?;

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(request_url)
        .header(USER_AGENT, "rust-web-api-client") // gh api requires a user-agent header
        .bearer_auth(token)
        .send()?;

    let users: Vec<User> = response.json()?;
    for user in users {
        println!("{} ({})", user.login, user.id);
    }
    Ok(())
}
