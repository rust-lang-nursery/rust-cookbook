use web::wiki;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let content = reqwest::Client::builder()
        .user_agent("rust-cookbook")
        .build()?
        .get("https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=raw")
        .send()
        .await?
        .text()
        .await?;

    println!("{:#?}", wiki::extract_links(content.as_str()));

    Ok(())
}
