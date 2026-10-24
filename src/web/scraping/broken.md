## Check a webpage for broken links

[![reqwest-badge]][reqwest] [![scraper-badge]][scraper] [![url-badge]][url] [![cat-net-badge]][cat-net]

`get_links` parses the page with [`Html::parse_document`] and calls
`get_base_url` to retrieve the base URL. If the document has a base tag,
get the href [`attr`] from the `base[href]` element, which
[`Selector::parse`] and [`Html::select`] find with a CSS selector.
[`Position::BeforePath`] of the original URL acts as a default.

Iterates through links in the document and creates a [`tokio::task::spawn`] task that will 
parse an individual link with [`url::ParseOptions`] and [`Url::parse`]). 
The task makes a request to the links with [reqwest] and verifies
[`StatusCode`].  Then the tasks `await` completion before ending the program.

```rust,edition2024
{{#include ../../../crates/web_scraping/src/broken.rs::84 }}
```

```rust,edition2024,no_run
{{#include ../../../crates/web_scraping/src/bin/broken_links.rs }}
```

[`attr`]: https://docs.rs/scraper/*/scraper/node/struct.Element.html#method.attr
[`Html::parse_document`]: https://docs.rs/scraper/*/scraper/html/struct.Html.html#method.parse_document
[`Html::select`]: https://docs.rs/scraper/*/scraper/html/struct.Html.html#method.select
[`Selector::parse`]: https://docs.rs/scraper/*/scraper/selector/struct.Selector.html#method.parse
[`Position::BeforePath`]: https://docs.rs/url/*/url/enum.Position.html#variant.BeforePath
[`StatusCode`]: https://docs.rs/reqwest/*/reqwest/struct.StatusCode.html
[`tokio::task::spawn`]: https://docs.rs/tokio/*/tokio/task/fn.spawn.html
[`url::Parse`]: https://docs.rs/url/*/url/struct.Url.html#method.parse
[`url::ParseOptions`]: https://docs.rs/url/*/url/struct.ParseOptions.html
