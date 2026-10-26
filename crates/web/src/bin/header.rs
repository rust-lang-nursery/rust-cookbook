use anyhow::Result;
use reqwest::Url;
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct HeadersEcho {
    pub headers: HashMap<String, String>,
}

fn main() -> Result<()> {
    let url = Url::parse_with_params(
        "http://httpbin.org/headers",
        &[("lang", "rust"), ("browser", "servo")],
    )?;

    let response = Client::new()
        .get(url)
        .header(USER_AGENT, "Rust-test-agent")
        .header(AUTHORIZATION, "Bearer my-token")
        .header("X-Powered-By", "Rust")
        .send()?;

    assert_eq!(
        response.url().as_str(),
        "http://httpbin.org/headers?lang=rust&browser=servo"
    );

    let out: HeadersEcho = response.json()?;
    assert_eq!(out.headers["User-Agent"], "Rust-test-agent");
    assert_eq!(out.headers["Authorization"], "Bearer my-token");
    assert_eq!(out.headers["X-Powered-By"], "Rust");

    Ok(())
}
