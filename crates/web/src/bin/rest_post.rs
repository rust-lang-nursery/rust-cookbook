use anyhow::Result;
use serde::Deserialize;
use serde_json::json;
use std::env;

#[derive(Deserialize)]
struct Gist {
    id: String,
    html_url: String,
}

fn main() -> Result<()> {
    let gh_user = env::var("GH_USER")?;
    let gh_pass = env::var("GH_PASS")?;

    let gist_body = json!({
    "description": "the description for this gist",
    "public": true,
    "files": {
         "main.rs": {
         "content": r#"fn main() { println!("hello world!");}"#
        }
    }});

    let request_url = "https://api.github.com/gists";
    let response = reqwest::blocking::Client::new()
        .post(request_url)
        .basic_auth(gh_user.clone(), Some(gh_pass.clone()))
        .json(&gist_body)
        .send()?;

    let gist: Gist = response.json()?;
    println!("Created {}", gist.html_url);

    let request_url = format!("{}/{}", request_url, gist.id);
    let response = reqwest::blocking::Client::new()
        .delete(&request_url)
        .basic_auth(gh_user, Some(gh_pass))
        .send()?;

    println!(
        "Gist {} deleted! Status code: {}",
        gist.id,
        response.status()
    );
    Ok(())
}
