## Extract all links from a webpage HTML

[![reqwest-badge]][reqwest] [![select-badge]][select] [![cat-net-badge]][cat-net]

Use [`reqwest::get`] to perform a HTTP GET request and then use
[`Document::from_read`] to parse the response into a HTML document.
[`find`] with the criteria of [`Name`] is "a" retrieves all links.
Call [`filter_map`] on the [`Selection`] retrieves URLs
from links that have the "href" [`attr`] (attribute).

```rust,edition2024,no_run
// cargo-deps: tokio="1", reqwest="0.11", select="0.6", thiserror="1"
mod links {
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
}

#[tokio::main]
async fn main() -> Result<(), links::LinkError> {
    let page_links = links::get_links("https://www.rust-lang.org/en-US/").await?;
    for link in page_links {
        println!("{}", link);
    }
    Ok(())
}
```

[`attr`]: https://docs.rs/select/*/select/node/struct.Node.html#method.attr
[`Document::from_read`]: https://docs.rs/select/*/select/document/struct.Document.html#method.from_read
[`filter_map`]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.filter_map
[`find`]: https://docs.rs/select/*/select/document/struct.Document.html#method.find
[`Name`]: https://docs.rs/select/*/select/predicate/struct.Name.html
[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`Selection`]: https://docs.rs/select/*/select/selection/struct.Selection.html
