## Check a webpage for broken links

[![reqwest-badge]][reqwest] [![select-badge]][select] [![url-badge]][url] [![cat-net-badge]][cat-net]

Call `get_base_url` to retrieve the base URL. If the document has a base tag,
get the href [`attr`] from base tag. [`Position::BeforePath`] of the original
URL acts as a default.

Iterate through links in the document and parse with [`url::ParseOptions`]
and [`Url::parse`]). Makes a request to the links with reqwest and verifies
[`StatusCode`].

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate reqwest;
extern crate select;
extern crate url;

use std::collections::HashSet;

use url::{Url, Position};
use reqwest::StatusCode;
use select::document::Document;
use select::predicate::Name;
#
# error_chain! {
#   foreign_links {
#       ReqError(reqwest::Error);
#       IoError(std::io::Error);
#       UrlParseError(url::ParseError);
#   }
# }

fn get_base_url(url: &Url, doc: &Document) -> Result<Url> {
    let base_tag_href = doc.find(Name("base")).filter_map(|n| n.attr("href")).nth(0);

    let base_url = base_tag_href.map_or_else(
        || Url::parse(&url[..Position::BeforePath]),
        Url::parse,
    )?;

    Ok(base_url)
}

fn check_link(url: &Url) -> Result<bool> {
    let res = reqwest::get(url.as_ref())?;

    Ok(res.status() != StatusCode::NOT_FOUND)
}

fn run() -> Result<()> {
    let url = Url::parse("https://www.rust-lang.org/en-US/")?;

    let res = reqwest::get(url.as_ref())?;
    let document = Document::from_read(res)?;

    let base_url = get_base_url(&url, &document)?;

    let base_parser = Url::options().base_url(Some(&base_url));

    let links: HashSet<Url> = document
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter_map(|link| base_parser.parse(link).ok())
        .collect();

    links
        .iter()
        .filter(|link| check_link(link).ok() == Some(false))
        .for_each(|x| println!("{} is broken.", x));

    Ok(())
}
#
# quick_main!(run);
```

[`attr`]: https://docs.rs/select/*/select/node/struct.Node.html#method.attr
[`Position::BeforePath`]: https://docs.rs/url/*/url/enum.Position.html#variant.BeforePath
[`StatusCode`]: https://docs.rs/reqwest/*/reqwest/struct.StatusCode.html
[`url::Parse`]: https://docs.rs/url/*/url/struct.Url.html#method.parse
[`url::ParseOptions`]: https://docs.rs/url/*/url/struct.ParseOptions.html
