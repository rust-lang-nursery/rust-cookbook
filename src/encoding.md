# Encoding

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Serialize and deserialize unstructured JSON][ex-json-value] | [![serde-json-badge]][serde-json] | [![cat-encoding-badge]][cat-encoding] |
| [Deserialize a TOML configuration file][ex-toml-config] | [![toml-badge]][toml] | [![cat-encoding-badge]][cat-encoding] |
| [Percent-encode a string][ex-percent-encode] | [![url-badge]][url] | [![cat-encoding-badge]][cat-encoding] |
| [Encode and decode hex][ex-hex-encode-decode] | [![data-encoding-badge]][data-encoding] | [![cat-encoding-badge]][cat-encoding] |

[ex-json-value]: #ex-json-value
<a name="ex-json-value"></a>
## Serialize and deserialize unstructured JSON

[![serde-json-badge]][serde-json] [![cat-encoding-badge]][cat-encoding]

The [`serde_json`] crate provides a [`from_str`] function to parse a `&str` of
JSON into a type of the caller's choice.

[`serde_json`]: https://docs.serde.rs/serde_json/
[`from_str`]: https://docs.serde.rs/serde_json/fn.from_str.html

Unstructured JSON can be parsed into a universal [`serde_json::Value`] type that
is able to represent any valid JSON data.

[`serde_json::Value`]: https://docs.serde.rs/serde_json/enum.Value.html

The example below shows a `&str` of JSON being parsed and then compared to what
we expect the parsed value to be. The expected value is declared using the
[`json!`] macro.

[`json!`]: https://docs.serde.rs/serde_json/macro.json.html

```rust
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate error_chain;

use serde_json::Value;

error_chain! {
    foreign_links {
        Json(serde_json::Error);
    }
}

fn run() -> Result<()> {
    let j = r#"{
                 "userid": 103609,
                 "verified": true,
                 "access_privileges": [
                   "user",
                   "admin"
                 ]
               }"#;

    let parsed: Value = serde_json::from_str(j)?;

    let expected = json!({
        "userid": 103609,
        "verified": true,
        "access_privileges": [
            "user",
            "admin"
        ]
    });

    assert_eq!(parsed, expected);

    Ok(())
}

quick_main!(run);
```

[ex-toml-config]: #ex-toml-config
<a name="ex-toml-config"></a>
## Deserialize a TOML configuration file

[![toml-badge]][toml] [![cat-encoding-badge]][cat-encoding]

Parse some TOML into a universal `toml::Value` that is able to represent any
valid TOML data.

```rust
extern crate toml;

#[macro_use]
extern crate error_chain;

use toml::Value;

error_chain! {
    foreign_links {
        Toml(toml::de::Error);
    }
}

fn run() -> Result<()> {
    let toml_content = r#"
          [package]
          name = "your_package"
          version = "0.1.0"
          authors = ["You! <you@example.org>"]

          [dependencies]
          serde = "1.0"
          "#;

    let package_info: Value = toml::from_str(toml_content)?;

    assert_eq!(package_info["dependencies"]["serde"].as_str(), Some("1.0"));
    assert_eq!(package_info["package"]["name"].as_str(), Some("your_package"));

    Ok(())
}

quick_main!(run);
```

Parse TOML into your own structs using Serde:

[![serde-json-badge]][serde-json] [![toml-badge]][toml] [![cat-encoding-badge]][cat-encoding]

```rust
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate toml;

#[macro_use]
extern crate error_chain;

use std::collections::HashMap;

#[derive(Deserialize)]
struct Config {
    package: Package,
    dependencies: HashMap<String, String>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}

error_chain! {
    foreign_links {
        Toml(toml::de::Error);
    }
}

fn run() -> Result<()> {
    let toml_content = r#"
          [package]
          name = "your_package"
          version = "0.1.0"
          authors = ["You! <you@example.org>"]

          [dependencies]
          serde = "1.0"
          "#;

    let package_info: Config = toml::from_str(toml_content)?;

    assert_eq!(package_info.package.name, "your_package");
    assert_eq!(package_info.package.version, "0.1.0");
    assert_eq!(package_info.package.authors, vec!["You! <you@example.org>"]);
    assert_eq!(package_info.dependencies["serde"], "1.0");

    Ok(())
}

quick_main!(run);
```

[ex-percent-encode]: #ex-percent-encode
<a name="ex-percent-encode"></a>
## Percent-encode a string

[![url-badge]][url] [![cat-encoding-badge]][cat-encoding]

Encode an input string with [percent-encoding] using the [`utf8_percent_encode`]
function from the `url` crate. Then decode using the [`percent_decode`]
function.

```rust
extern crate url;

#[macro_use]
extern crate error_chain;

use url::percent_encoding::{utf8_percent_encode, percent_decode, DEFAULT_ENCODE_SET};

error_chain! {
    foreign_links {
        Utf8(std::str::Utf8Error);
    }
}

fn run() -> Result<()> {
    let input = "confident, productive systems programming";

    let iter = utf8_percent_encode(input, DEFAULT_ENCODE_SET);
    let encoded: String = iter.collect();
    assert_eq!(encoded, "confident,%20productive%20systems%20programming");

    let iter = percent_decode(encoded.as_bytes());
    let decoded = iter.decode_utf8()?;
    assert_eq!(decoded, "confident, productive systems programming");

    Ok(())
}

quick_main!(run);
```

The encode set defines which bytes (in addition to non-ASCII and controls) need
to be percent-encoded. The choice of this set depends on context. For example,
`?` needs to be encoded in a URL path but not in a query string.

The return value of encoding is an iterator of `&str` slices which can be
collected into a `String`.


[ex-hex-encode-decode]: #ex-hex-encode-decode
<a name="ex-hex-encode-decode"></a>
## Encode and decode hex

[![data-encoding-badge]][data-encoding] [![cat-encoding-badge]][cat-encoding]

The [`data_encoding`] crate provides a `HEXUPPER::encode` method which
takes a `&[u8]` and returns a `String` containing the hexadecimal
representation of the data.

Similarly, a `HEXUPPER::decode` method is provided which takes a `&[u8]` and
returns a `Vec<u8>` if the input data is successfully decoded.

[`data_encoding`]: https://github.com/ia0/data-encoding

The example below shows a `&[u8]` of data being converted to its hexadecimal
representation and then being compared to its expected value. The returned
hex `String` is then converted back to its original representation and is
compared to the original value provided.

```rust
extern crate data_encoding;

#[macro_use]
extern crate error_chain;

use data_encoding::{HEXUPPER, DecodeError};

error_chain! {
    foreign_links {
        Decode(DecodeError);
    }
}

fn run() -> Result<()> {
    let original = b"The quick brown fox jumps over the lazy dog.";
    let expected = "54686520717569636B2062726F776E20666F78206A756D7073206F76\
        657220746865206C617A7920646F672E";
    
    let encoded = HEXUPPER.encode(original);
    assert_eq!(encoded, expected);

    let decoded = HEXUPPER.decode(&encoded.into_bytes())?;
    assert_eq!(&decoded[..], &original[..]);

    Ok(())
}

quick_main!(run);
```

<!-- Categories -->

[cat-encoding-badge]: https://img.shields.io/badge/-encoding-red.svg
[cat-encoding]: https://crates.io/categories/encoding

<!-- Crates -->

[serde-json-badge]: https://img.shields.io/crates/v/serde_json.svg?label=serde_json
[serde-json]: https://docs.serde.rs/serde_json/
[toml-badge]: https://img.shields.io/crates/v/toml.svg?label=toml
[toml]: https://docs.rs/toml/
[url-badge]: https://img.shields.io/crates/v/url.svg?label=url
[url]: https://docs.rs/url/
[data-encoding-badge]: https://img.shields.io/crates/v/data-encoding.svg?label=url
[data-encoding]: https://github.com/ia0/data-encoding

<!-- Reference -->

[`percent_decode`]: https://docs.rs/url/1.*/url/percent_encoding/fn.percent_decode.html
[`utf8_percent_encode`]: https://docs.rs/url/1.*/url/percent_encoding/fn.utf8_percent_encode.html
[percent-encoding]: https://en.wikipedia.org/wiki/Percent-encoding
