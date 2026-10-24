use reqwest::StatusCode;
use scraper::{Html, Selector};
use std::collections::HashSet;
use thiserror::Error;
use url::{Position, Url};

#[derive(Error, Debug)]
pub enum BrokenError {
    #[error("Reqwest error: {0}")]
    ReqError(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}

pub struct CategorizedUrls {
    pub ok: Vec<String>,
    pub broken: Vec<String>,
}

enum Link {
    GoodLink(Url),
    BadLink(Url),
}

fn get_base_url(url: &Url, document: &Html) -> Result<Url, BrokenError> {
    let selector = Selector::parse("base[href]").expect("valid CSS selector");
    let base_tag_href = document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .next();
    let base_url =
        base_tag_href.map_or_else(|| Url::parse(&url[..Position::BeforePath]), Url::parse)?;
    Ok(base_url)
}

fn get_links(url: &Url, body: &str) -> Result<HashSet<Url>, BrokenError> {
    let document = Html::parse_document(body);
    let base_url = get_base_url(url, &document)?;
    let base_parser = Url::options().base_url(Some(&base_url));
    let selector = Selector::parse("a[href]").expect("valid CSS selector");
    let links = document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .filter_map(|link| base_parser.parse(link).ok())
        .collect();
    Ok(links)
}

async fn check_link(url: &Url) -> Result<bool, BrokenError> {
    let res = reqwest::get(url.as_ref()).await?;
    Ok(res.status() != StatusCode::NOT_FOUND)
}

pub async fn check(site: &str) -> Result<CategorizedUrls, BrokenError> {
    let url = Url::parse(site)?;
    let res = reqwest::get(url.as_ref()).await?.text().await?;
    let links = get_links(&url, &res)?;
    let mut tasks = vec![];
    let mut ok = vec![];
    let mut broken = vec![];

    for link in links {
        tasks.push(tokio::spawn(async move {
            if check_link(&link).await.unwrap_or(false) {
                Link::GoodLink(link)
            } else {
                Link::BadLink(link)
            }
        }));
    }

    for task in tasks {
        match task.await? {
            Link::GoodLink(link) => ok.push(link.to_string()),
            Link::BadLink(link) => broken.push(link.to_string()),
        }
    }

    Ok(CategorizedUrls { ok, broken })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_links_against_the_base_tag() -> Result<(), BrokenError> {
        let page = Url::parse("https://example.com/a/b.html")?;
        let html = r#"<base href="https://cdn.example.org/root/">
            <a href="x.html">x</a><a href="https://other.net/y">y</a><a>none</a>"#;
        let links = get_links(&page, html)?;
        let expected: HashSet<Url> = ["https://cdn.example.org/root/x.html", "https://other.net/y"]
            .iter()
            .map(|s| Url::parse(s))
            .collect::<Result<_, _>>()?;
        assert_eq!(links, expected);
        Ok(())
    }

    #[test]
    fn defaults_to_the_site_root_without_a_base_tag() -> Result<(), BrokenError> {
        let page = Url::parse("https://example.com/a/b.html")?;
        let links = get_links(&page, r#"<a href="/c">c</a>"#)?;
        assert!(links.contains(&Url::parse("https://example.com/c")?));
        Ok(())
    }
}
