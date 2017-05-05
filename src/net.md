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

Parses a URL from a string to a `Url` type and then prints it to the console.

```rust
extern crate url;

use url::Url;

fn main() {
    let url = Url::parse(
        "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open"
    ).unwrap();
    println!("url: {}", url.as_str());
```

[ex-url-base]: #ex-url-base
<a name="ex-url-base"></a>
## Create a base URL by removing path segments

[Write me!](https://github.com/brson/rust-cookbook/issues/34)

[ex-url-new-from-base]: #ex-url-new-from-base
<a name="ex-url-new-from-base"></a>
## Create new URLs from a base URL

[Write me!](https://github.com/brson/rust-cookbook/issues/35)

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
