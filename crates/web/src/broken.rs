use thiserror::Error;
use reqwest::StatusCode;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
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

async fn get_base_url(url: &Url, doc: &Document) -> Result<Url, BrokenError> {
  let base_tag_href = doc.find(Name("base")).filter_map(|n| n.attr("href")).nth(0);
  let base_url =
    base_tag_href.map_or_else(|| Url::parse(&url[..Position::BeforePath]), Url::parse)?;
  Ok(base_url)
}

async fn check_link(url: &Url) -> Result<bool, BrokenError> {
  let res = reqwest::get(url.as_ref()).await?;
  Ok(res.status() != StatusCode::NOT_FOUND)
}

pub async fn check(site: &str) -> Result<CategorizedUrls, BrokenError> {
  let url = Url::parse(site)?;
  let res = reqwest::get(url.as_ref()).await?.text().await?;
  let document = Document::from(res.as_str());
  let base_url = get_base_url(&url, &document).await?;
  let base_parser = Url::options().base_url(Some(&base_url));
  let links: HashSet<Url> = document
    .find(Name("a"))
    .filter_map(|n| n.attr("href"))
    .filter_map(|link| base_parser.parse(link).ok())
    .collect();
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

