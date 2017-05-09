# Networking

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Parse a URL from a string to a `Url` type][ex-url-parse] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Create a base URL by removing path segments][ex-url-base] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Create new URLs from a base URL][ex-url-new-from-base] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Extract the URL origin (scheme / host / port)][ex-url-origin] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Remove fragment identifiers and query pairs from a URL][ex-url-rm-frag] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Serialize a `Url`][ex-url-serialize] | [![url-badge]][url] [![serde-badge]][serde] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]|
| [Make a HTTP GET request][ex-url-basic] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |

[ex-url-parse]: #ex-url-parse
<a name="ex-url-parse"/>
## Parse a URL from a string to a `Url` type

[![url-badge]][url] [![cat-net-badge]][cat-net]

The [`parse`] method from the `url` crate validates and parses a `&str` into a
[`Url`] struct. The input string may be malformed so this method returns
`Result<Url, ParseError>`.

Once the URL has been parsed, it can be used with all of the methods on the
`Url` type.

The URL in this code parses successfully, but swapping it out for a malformed
URL will print a message containing an explanation of what went wrong.

```rust
extern crate url;

use url::Url;

fn main() {
    let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";

    match Url::parse(s) {
        Ok(url) => {
            println!("Successfully parsed the URL.");
            println!("The path part of the URL is: {}", url.path());
        }
        Err(err) => {
            println!("Failed to parse the URL: {}", err);
        }
    }
}
```

[ex-url-base]: #ex-url-base
<a name="ex-url-base"></a>
## Create a base URL by removing path segments

[![url-badge]][url] [![cat-net-badge]][cat-net]

```rust
extern crate url;

use url::{Url, ParseError};

#[macro_use]
extern crate error_chain;

error_chain! {
    errors {
        CannotBeABase
    }
    foreign_links {
        Parse(ParseError);
    }
}

fn main() {
    let s = "https://github.com/rust-lang/cargo?asdf";

    match base_url(s) {
        Ok(base) => {
            assert_eq!(base.as_str(), "https://github.com/");
            println!("The base of the URL is: {}", base);
        }
        Err(err) => {
            println!("Failed to extract base URL: {}", err);
        }
    }
}

/// Returns the base of the given URL - the part not including any path segments
/// and query parameters.
fn base_url(full: &str) -> Result<Url> {
    let mut url = Url::parse(full)?;

    // Clear path segments.
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => {
            // Certain URLs cannot be turned into a base URL.
            return Err(Error::from_kind(ErrorKind::CannotBeABase));
        }
    }

    // Clear query parameters.
    url.set_query(None);

    Ok(url)
}
```

[ex-url-new-from-base]: #ex-url-new-from-base
<a name="ex-url-new-from-base"></a>
## Create new URLs from a base URL

[![url-badge]][url] [![cat-net-badge]][cat-net]

The [`join`] method creates a new URL from a base and relative path.

```rust
extern crate url;

use url::{Url, ParseError};

fn main() {
    let path = "/rust-lang/cargo";

    match build_github_url(path) {
        Ok(url) => {
            assert_eq!(url.as_str(), "https://github.com/rust-lang/cargo");
            println!("The joined URL is: {}", url);
        }
        Err(err) => {
            println!("Failed to build GitHub URL: {}", err);
        }
    }
}

fn build_github_url(path: &str) -> Result<Url, ParseError> {
    // Hardcoded in our program. Caller's path will be joined to this.
    const GITHUB: &'static str = "https://github.com";

    let base = Url::parse(GITHUB).expect("hardcoded URL is known to be valid");
    base.join(path)
}
```

[ex-url-origin]: #ex-url-origin
<a name="ex-url-origin"></a>
## Extract the URL origin (scheme / host / port)

[![url-badge]][url] [![cat-net-badge]][cat-net]

The [`Url`] struct exposes various methods to extract information about the URL
it represents.

```rust
extern crate url;

use url::{Url, Host};

fn main() {
    let s = "ftp://rust-lang.org/examples";

    match Url::parse(s) {
        Ok(url) => {
            assert_eq!(url.scheme(), "ftp");
            assert_eq!(url.host(), Some(Host::Domain("rust-lang.org")));
            assert_eq!(url.port_or_known_default(), Some(21));
            println!("The origin is as expected!");
        }
        Err(err) => {
            println!("Failed to parse the URL: {}", err);
        }
    }
}
```

The same result can be obtained using the [`origin`] method as well.

```rust
extern crate url;

use url::{Url, Origin, Host};

fn main() {
    let s = "ftp://rust-lang.org/examples";

    match Url::parse(s) {
        Ok(url) => {
            let expected_scheme = "ftp".to_owned();
            let expected_host = Host::Domain("rust-lang.org".to_owned());
            let expected_port = 21;
            let expected = Origin::Tuple(expected_scheme, expected_host, expected_port);

            let origin = url.origin();
            assert_eq!(origin, expected);
            println!("The origin is as expected!");
        }
        Err(err) => {
            println!("Failed to parse the URL: {}", err);
        }
    }
}
```

[ex-url-rm-frag]: #ex-url-rm-frag
<a name="ex-url-rm-frag"></a>
## Remove fragment identifiers and query pairs from a URL

[![url-badge]][url] [![cat-net-badge]][cat-net]

Once [`Url`] is parsed it can be sliced with [`url::Position`] to strip unneded URL parts.

```rust
extern crate url;

use url::{Url, Position};

fn run() -> Result<(), url::ParseError> {
    let parsed = Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open")?;
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("cleaned: {}", cleaned);
    Ok(())
}

fn main() {
    run().unwrap();
}
```

[ex-url-serialize]: #ex-url-serialize
<a name="ex-url-serialize"></a>
## Serialize a `Url`

[Write me!](https://github.com/brson/rust-cookbook/issues/38)

[ex-url-basic]: #ex-url-basic
<a name="ex-url-basic"></a>
## Make a HTTP GET request

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

The [`reqwest::get`] function parses the supplied url and makes a
synchronus HTTP GET request. Obtained [`reqwest::Response`]
status and headders are printed. HTTP response body is read into an allocated [`String`] via [`read_to_string`].

```rust,no_run
extern crate reqwest;
#[macro_use]
extern crate error_chain;

use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpReqest(reqwest::Error);
    }
}

fn run() -> Result<()> {
    let mut res = reqwest::get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}

quick_main!(run);
```
<!-- Categories -->

[cat-encoding-badge]: https://img.shields.io/badge/-encoding-red.svg
[cat-encoding]: https://crates.io/categories/encoding
[cat-net-badge]: https://img.shields.io/badge/-net-red.svg
[cat-net]: https://crates.io/categories/network-programming

<!-- Crates -->

[reqwest-badge]: https://img.shields.io/crates/v/reqwest.svg?label=reqwest
[reqwest]: https://docs.rs/reqwest/
[serde-badge]: https://img.shields.io/crates/v/serde.svg?label=serde
[serde]: https://docs.rs/serde/
[url-badge]: https://img.shields.io/crates/v/url.svg?label=url
[url]: https://docs.rs/url/

<!-- Reference -->

[`Url`]: https://docs.rs/url/1.*/url/struct.Url.html
[`parse`]: https://docs.rs/url/1.*/url/struct.Url.html#method.parse
[`url::Position`]: https://docs.rs/url/*/url/enum.Position.html
[`origin`]: https://docs.rs/url/1.*/url/struct.Url.html#method.origin
[`join`]: https://docs.rs/url/1.*/url/struct.Url.html#method.join
[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`reqwest::Response`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html
[`read_to_string`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
