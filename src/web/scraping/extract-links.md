## Extract all links from a webpage HTML

[![reqwest-badge]][reqwest] [![scraper-badge]][scraper] [![tokio-badge]][tokio] [![thiserror-badge]][thiserror] [![cat-net-badge]][cat-net]

Use [`reqwest::get`] to perform a HTTP GET request and then use
[`Html::parse_document`] to parse the response into a HTML document.
[`Selector::parse`] compiles the CSS selector `a[href]`, which matches every
link that has an "href" attribute. [`Html::select`] yields the matching
elements, and [`filter_map`] retrieves each URL with [`attr`] on the
element's [`ElementRef::value`].

> This recipe requires the [`macros`] and [`rt-multi-thread`] feature flags of `tokio` to be enabled in `Cargo.toml`.

```rust,edition2024
{{#include ../../../crates/web_scraping/src/links.rs::25 }}
```

```rust,edition2024,no_run
{{#include ../../../crates/web_scraping/src/bin/extract_links.rs }}
```

[`attr`]: https://docs.rs/scraper/*/scraper/node/struct.Element.html#method.attr
[`filter_map`]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.filter_map
[`Html::parse_document`]: https://docs.rs/scraper/*/scraper/html/struct.Html.html#method.parse_document
[`Html::select`]: https://docs.rs/scraper/*/scraper/html/struct.Html.html#method.select
[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`Selector::parse`]: https://docs.rs/scraper/*/scraper/selector/struct.Selector.html#method.parse
[`ElementRef::value`]: https://docs.rs/scraper/*/scraper/element_ref/struct.ElementRef.html#method.value
[`macros`]: https://docs.rs/crate/tokio/*/features#macros
[`rt-multi-thread`]: https://docs.rs/crate/tokio/*/features#rt-multi-thread
