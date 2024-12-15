## Extract all unique links from a MediaWiki markup

[![reqwest-badge]][reqwest] [![regex-badge]][regex] [![cat-net-badge]][cat-net]

Pull the source of a MediaWiki page using [`reqwest::get`] and then
look for all entries of internal and external links with
[`Regex::captures_iter`]. Using [`Cow`] avoids excessive [`String`] allocations.

MediaWiki link syntax is described [here][MediaWiki link syntax].  The calling
function will retain the whole document, and links will be returned as slice
references to the original document.

```rust,edition2024,no_run
mod wiki {
  {{#include ../../../crates/web/src/wiki.rs}}
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
