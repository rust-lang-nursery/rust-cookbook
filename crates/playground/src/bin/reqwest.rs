fn main() -> Result<(), reqwest::Error> {
    // The playground has no network access, so this builds a request without sending it.
    // Add `.send().await` inside an async runtime (see the tokio starter) to send it.
    let request = reqwest::Client::new()
        .get("https://example.com/api?q=rust")
        .header("User-Agent", "cookbook")
        .build()?;

    println!("{} {}", request.method(), request.url());
    Ok(())
}
