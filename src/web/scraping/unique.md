## Extract all unique links from a MediaWiki markup

[![reqwest-badge]][reqwest] [![regex-badge]][regex] [![tokio-badge]][tokio] [![cat-net-badge]][cat-net]

Pull the source of a MediaWiki page using [`reqwest::get`] and then
look for all entries of internal and external links with
[`Regex::captures_iter`]. Using [`Cow`] avoids excessive [`String`] allocations.

MediaWiki link syntax is described [here][MediaWiki link syntax].  The calling
function will retain the whole document, and links will be returned as slice
references to the original document. The request sets a user agent, since
Wikipedia rejects requests without one.

> This recipe requires the [`macros`] and [`rt-multi-thread`] feature flags of `tokio` to be enabled in `Cargo.toml`.

```rust
{{#include ../../../crates/web/src/wiki.rs }}
```

```rust,no_run
{{#include ../../../crates/web/src/bin/unique.rs }}
```

[`Cow`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`Regex::captures_iter`]: https://docs.rs/regex/*/regex/struct.Regex.html#method.captures_iter
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
[`LazyLock`]: https://doc.rust-lang.org/std/sync/struct.LazyLock.html

[MediaWiki link syntax]: https://www.mediawiki.org/wiki/Help:Links
[`macros`]: https://docs.rs/crate/tokio/*/features#macros
[`rt-multi-thread`]: https://docs.rs/crate/tokio/*/features#rt-multi-thread
