pub mod paginated;
pub mod wiki;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_dependencies() -> reqwest::Result<()> {
        for dep in paginated::ReverseDependencies::of("serde")?.take(5) {
            let dependency = dep?;
            println!("{} depends on {}", dependency.id, dependency.crate_id);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_wiki() -> anyhow::Result<()> {
        let content = reqwest::get(
            "https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=raw",
        )
        .await?
        .text()
        .await?;

        println!("{:#?}", wiki::extract_links(content.as_str()));

        Ok(())
    }
}
