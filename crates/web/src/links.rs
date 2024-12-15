use thiserror::Error;
use select::document::Document;
use select::predicate::Name;

#[derive(Error, Debug)]
pub enum LinkError {
    #[error("Reqwest error: {0}")]
    ReqError(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub async fn get_links(page: &str) -> Result<Vec<Box<str>>, LinkError> {
  let res = reqwest::get(page)
    .await?
    .text()
    .await?;

  let links = Document::from(res.as_str())
    .find(Name("a"))
    .filter_map(|node| node.attr("href"))
    .into_iter()
    .map(|link| Box::<str>::from(link.to_string()))
    .collect();

  Ok(links)
}

