## Remove fragment identifiers and query pairs from a URL

[![url-badge]][url] [![cat-net-badge]][cat-net]

Parses [`Url`] and slices it with [`url::Position`] to strip unneeded URL parts.

```rust

extern crate url;

use url::{Url, Position, ParseError};

fn main() -> Result<(), ParseError> {
    let parsed = Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open")?;
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("cleaned: {}", cleaned);
    Ok(())
}
```

[`url::Position`]: https://docs.rs/url/*/url/enum.Position.html
[`Url`]: https://docs.rs/url/*/url/struct.Url.html
