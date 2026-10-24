use web_scraping_example::links;

#[tokio::main]
async fn main() -> Result<(), links::LinkError> {
    let page_links = links::get_links("https://www.rust-lang.org/en-US/").await?;
    for link in page_links {
        println!("{}", link);
    }
    Ok(())
}
