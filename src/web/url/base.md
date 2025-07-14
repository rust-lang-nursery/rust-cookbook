## Create a base URL by removing path segments

[![url-badge]][url] [![cat-net-badge]][cat-net]

A base URL includes a protocol and a domain.  Base URLs have no folders,
files or query strings.  Each of those items are stripped out of the given
URL.  [`PathSegmentsMut::clear`] removes paths and [`Url::set_query`] removes
query string.

```rust,edition2021
use anyhow::{Result, anyhow};
use url::Url;

fn main() -> Result<()> {
    let full = "https://github.com/rust-lang/cargo?asdf";

    let url = Url::parse(full)?;
    let base = base_url(url)?;

    assert_eq!(base.as_str(), "https://github.com/");
    println!("The base of the URL is: {}", base);

    Ok(())
}

fn base_url(mut url: Url) -> Result<Url> {
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => {
            return Err(anyhow!("Cannot be a base URL"));
        }
    }

    url.set_query(None);

    Ok(url)
}
```

[`PathSegmentsMut::clear`]: https://docs.rs/url/*/url/struct.PathSegmentsMut.html#method.clear
[`Url::set_query`]: https://docs.rs/url/*/url/struct.Url.html#method.set_query
