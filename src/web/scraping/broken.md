## Check a webpage for broken links

[![reqwest-badge]][reqwest] [![select-badge]][select] [![url-badge]][url] [![cat-net-badge]][cat-net]

Call `get_base_url` to retrieve the base URL. If the document has a base tag,
get the href [`attr`] from base tag. [`Position::BeforePath`] of the original
URL acts as a default.

Iterates through links in the document and creates a [`tokio::task::spawn`] task that will 
parse an individual link with [`url::ParseOptions`] and [`Url::parse`]). 
The task makes a request to the links with [reqwest] and verifies
[`StatusCode`].  Then the tasks `await` completion before ending the program.

```rust,edition2024,ignore
// cargo-deps: tokio="1", reqwest="0.11", select="0.6", thiserror="1", url="2", anyhow="1"
mod broken {
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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let categorized = broken::check("https://www.rust-lang.org/en-US/").await?;
    println!("OK: {:?}", categorized.ok);
    println!("Broken: {:?}", categorized.broken);
    Ok(())
}
```

[`attr`]: https://docs.rs/select/*/select/node/struct.Node.html#method.attr
[`Position::BeforePath`]: https://docs.rs/url/*/url/enum.Position.html#variant.BeforePath
[`StatusCode`]: https://docs.rs/reqwest/*/reqwest/struct.StatusCode.html
[`tokio::task::spawn`]: https://docs.rs/tokio/*/tokio/task/fn.spawn.html
[`url::Parse`]: https://docs.rs/url/*/url/struct.Url.html#method.parse
[`url::ParseOptions`]: https://docs.rs/url/*/url/struct.ParseOptions.html
