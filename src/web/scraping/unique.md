## Extract all unique links from a MediaWiki markup

[![reqwest-badge]][reqwest] [![regex-badge]][regex] [![cat-net-badge]][cat-net]

Pull the source of a MediaWiki page using [`reqwest::get`] and then
look for all entries of internal and external links with
[`Regex::captures_iter`]. Using [`Cow`] avoids excessive [`String`] allocations.

MediaWiki link syntax is described [here][MediaWiki link syntax].  The calling
function will retain the whole document, and links will be returned as slice
references to the original document.

```rust,edition2021,no_run
// cargo-deps: tokio="1", reqwest="0.11", regex="1", anyhow="1"
mod wiki {
  use regex::Regex;
  use std::borrow::Cow;
  use std::collections::HashSet;
  use std::sync::LazyLock;

  pub fn extract_links(content: &str) -> HashSet<Cow<str>> {
    static WIKI_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(
        r"(?x)
                  \[\[(?P<internal>[^\[\]|]*)[^\[\]]*\]\]    # internal links
                  |
                  (url=|URL\||\[)(?P<external>http.*?)[ \|}] # external links
              "
      )
      .unwrap()
    );

    let links: HashSet<_> = WIKI_REGEX
      .captures_iter(content)
      .map(|c| match (c.name("internal"), c.name("external")) {
          (Some(val), None) => Cow::from(val.as_str()),
          (None, Some(val)) => Cow::from(val.as_str()),
          _ => unreachable!(),
      })
      .collect::<HashSet<_>>();

    links
  }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let content = reqwest::get(
    "https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=raw",
  )
  .await?
  .text()
  .await?;

  println!("{:#?}", wiki::extract_links(content.as_str()));

  Ok(())
}

```

[`Cow`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`Regex::captures_iter`]: https://docs.rs/regex/*/regex/struct.Regex.html#method.captures_iter
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
[`LazyLock`]: https://doc.rust-lang.org/std/sync/struct.LazyLock.html

[MediaWiki link syntax]: https://www.mediawiki.org/wiki/Help:Links
