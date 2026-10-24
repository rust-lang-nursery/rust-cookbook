use web_scraping_example::broken;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let categorized = broken::check("https://www.rust-lang.org/en-US/").await?;
    println!("OK: {:?}", categorized.ok);
    println!("Broken: {:?}", categorized.broken);
    Ok(())
}
