mod broken;
mod paginated;
mod links;

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
    async fn test_links() -> Result<(), links::LinkError> {
        let page_links = links::get_links("https://rust-lang-nursery.github.io/rust-cookbook/").await?;
        for link in page_links {
            println!("{}", link);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_broken() -> Result<(), broken::BrokenError> {
        let categorized = broken::check("https://rust-lang-nursery.github.io/rust-cookbook/web/scraping.html").await?;
        println!("OK: {:?}", categorized.ok);
        println!("Broken: {:?}", categorized.broken);
        Ok(())
    }
}
