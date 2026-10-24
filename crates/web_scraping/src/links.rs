use scraper::{Html, Selector};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinkError {
    #[error("Reqwest error: {0}")]
    ReqError(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

fn extract_links(html: &str) -> Vec<Box<str>> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a[href]").expect("valid CSS selector");
    document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .map(Box::<str>::from)
        .collect()
}

pub async fn get_links(page: &str) -> Result<Vec<Box<str>>, LinkError> {
    let res = reqwest::get(page).await?.text().await?;
    Ok(extract_links(&res))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_only_anchors_with_href() {
        let html = r#"<a href="/one">1</a><a name="x">2</a><a href="https://example.com/">3</a>"#;
        let links = extract_links(html);
        assert_eq!(links, vec!["/one".into(), "https://example.com/".into()]);
    }
}
