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

The [`Url`](https://docs.rs/url/struct.Url.html) struct provides the [`parse`](https://docs.rs/url/struct.Url.html#method.parse) method to build a URL from a &str which returns a `Result<Url, ParseError>`.

```rust
extern crate url;
use url::Url;

fn main() {
    let url_string = "data:text/plain,Hello?World#";
    let url = Url::parse(url_string).unwrap();
    
    assert_eq!(url.as_str(), url_string)
}
```
For documentation of `ParseError` see [this](https://docs.rs/url/enum.ParseError.html).

[ex-url-base]: #ex-url-base
<a name="ex-url-base"></a>
## Create a base URL by removing path segments

[Write me!](https://github.com/brson/rust-cookbook/issues/34)

[ex-url-new-from-base]: #ex-url-new-from-base
<a name="ex-url-new-from-base"></a>
## Create new URLs from a base URL

The [`Url`](https://docs.rs/url/struct.Url.html) struct provides the [`join`](https://docs.rs/url/struct.Url.html#method.join) method to create a new URL from a base.

```rust
extern crate url;
use url::Url;

fn main() {
    let this_document = Url::parse("http://servo.github.io/rust-url/url/index.html").unwrap();
    let css_url = this_document.join("../main.css").unwrap();
    
    assert_eq!(css_url.as_str(), "http://servo.github.io/rust-url/main.css")
}
```

[ex-url-origin]: #ex-url-origin
<a name="ex-url-origin"></a>
## Extract the URL origin (scheme / host / port)
The [`Url`](https://docs.rs/url/struct.Url.html) struct various methods to get information about the URL, here is an example where we retrieve it's origin.

```rust
extern crate url;
use url::{Url, Host};

fn main() {
    let url = Url::parse("ftp://example.com/foo").unwrap();
    assert!(url.scheme() == "ftp");
    assert!(url.host() == Some(Host::Domain("example.com".into())));
    assert!(url.port_or_known_default() == Some(21));
}
```

The same result can be obtained using the [`origin`](https://docs.rs/url/struct.Url.html#method.origin) method, which returns an [`Origin`](https://docs.rs/url/enum.Origin.html) enumerator.

```rust
extern crate url;
use url::{Url, Origin, Host};

fn main() {
    let url = Url::parse("ftp://example.com/foo").unwrap();
    assert_eq!(
        url.origin(),
        Origin::Tuple(
                "ftp".into(),
                 Host::Domain("example.com".into()),
                 21
        )
    );
}
```

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
