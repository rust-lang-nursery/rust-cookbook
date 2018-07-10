## Remove fragment identifiers and query pairs from a URL

[![url-badge]][url] [![cat-net-badge]][cat-net]

Parses [`Url`] and slices it with [`url::Position`] to strip unneeded URL parts.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate url;

use url::{Url, Position};
#
# error_chain! {
#     foreign_links {
#         UrlParse(url::ParseError);
#     }
# }

fn run() -> Result<()> {
    let parsed = Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open")?;
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("cleaned: {}", cleaned);
    Ok(())
}
#
# quick_main!(run);
```

[`url::Position`]: https://docs.rs/url/*/url/enum.Position.html
[`Url`]: https://docs.rs/url/*/url/struct.Url.html
