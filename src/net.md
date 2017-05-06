# Networking

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Parse a URL from a string to a `Url` type][ex-url-parse] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Create a base URL by removing path segments][ex-url-base] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Create new URLs from a base URL][ex-url-new-from-base] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Extract the URL origin (scheme / host / port)][ex-url-origin] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Remove fragment identifiers and query pairs from a URL][ex-url-rm-frag] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Serialize a `Url`][ex-url-serialize] | [![url-badge]][url] [![serde-badge]][serde] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]|
| [Make a HTTP GET request after parsing a URL][ex-url-reqwest] | [![reqwest-badge]][reqwest] [![url-badge]][url] | [![cat-net-badge]][cat-net] |

[ex-url-parse]: #ex-url-parse
<a name="ex-url-parse"/>
## Parse a URL from a string to a `Url` type

[![url-badge]][url] [![cat-net-badge]][cat-net]

Parses a URL from a string to a `Url` type and then prints it to the console. If the string doesn't parse as a `Url`, then you'll get a panic.

```rust
extern crate url;

use url::Url;

fn main() {
    match Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open") {
        Ok(url) => println!("url: {}", url),
        Err(error) => panic!("Errored parsing url: {}", error),
    };
}
```

[ex-url-base]: #ex-url-base
<a name="ex-url-base"></a>
## Create a base URL by removing path segments

[![url-badge]][url]
```rust
extern crate url;

use url::{ParseError as UrlParseError, Url};

#[derive(Debug)]
enum Error {
    Parse(UrlParseError),
    PathSegment,
}

impl From<UrlParseError> for Error {
    fn from(error: UrlParseError) -> Error {
        Error::Parse(error)
    }
}

impl From<()> for Error {
    fn from(_error: ()) -> Error {
        Error::PathSegment
    }
}

fn main() {
    match parse_base_url("https://github.com/rust-lang/cargo") {
        Ok(base_url) => assert_eq!(base_url.as_str(), "https://github.com/"),
        Err(error) => panic!("Errored parsing url: {:?}", error),
    }
}

fn parse_base_url(url: &str) -> Result<Url, Error> {
    let mut url = Url::parse(url)?;
    url.path_segments_mut()?.clear();
    Ok(url)
}
```

[ex-url-new-from-base]: #ex-url-new-from-base
<a name="ex-url-new-from-base"></a>
## Create new URLs from a base URL

[![url-badge]][url]
```rust
extern crate url;

use url::{ParseError, Url};

const BASE_GITHUB_URL: &'static str = "https://github.com";

fn main() {
    match build_github_url("rust-lang/cargo") {
        Ok(url) => assert_eq!(url.as_str(), "https://github.com/rust-lang/cargo"),
        Err(error) => panic!("Errored building url: {}", error),
    }
}

fn build_github_url(path: &str) -> Result<Url, ParseError> {
    let base_url = Url::parse(BASE_GITHUB_URL)?;
    base_url.join(path)
}
```

[ex-url-origin]: #ex-url-origin
<a name="ex-url-origin"></a>
## Extract the URL origin (scheme / host / port)

[Write me!](https://github.com/brson/rust-cookbook/issues/36)

[ex-url-rm-frag]: #ex-url-rm-frag
<a name="ex-url-rm-frag"></a>
## Remove fragment identifiers and query pairs from a URL

[Write me!](https://github.com/brson/rust-cookbook/issues/37)

[ex-url-serialize]: #ex-url-serialize
<a name="ex-url-serialize"></a>
## Serialize a `Url`

[Write me!](https://github.com/brson/rust-cookbook/issues/38)

[ex-url-reqwest]: #ex-url-reqwest
<a name="ex-url-reqwest"></a>
## Make a HTTP GET request after parsing a URL

[Write me!](https://github.com/brson/rust-cookbook/issues/39)

<!-- Categories -->

[cat-encoding-badge]: https://img.shields.io/badge/-encoding-red.svg
[cat-encoding]: https://crates.io/categories/encoding
[cat-net-badge]: https://img.shields.io/badge/-net-red.svg
[cat-net]: https://crates.io/categories/network-programming

<!-- Crates -->

[reqwest-badge]: https://img.shields.io/crates/v/reqwest.svg?label=reqwest
[reqwest]: https://docs.rs/url/
[serde-badge]: https://img.shields.io/crates/v/serde.svg?label=serde
[serde]: https://docs.rs/serde/
[url-badge]: https://img.shields.io/crates/v/url.svg?label=url
[url]: https://docs.rs/url/
