# Encoding

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Serialize and deserialize unstructured JSON][ex-json-value] | [![serde-json-badge]][serde-json] | [![cat-encoding-badge]][cat-encoding] |
| [Deserialize a TOML configuration file][ex-toml-config] | [![toml-badge]][toml] | [![cat-encoding-badge]][cat-encoding] |
| [Percent-encode a string][ex-percent-encode] | [![url-badge]][url] | [![cat-encoding-badge]][cat-encoding] |
| [Encode a string as application/x-www-form-urlencoded][ex-urlencoded] | [![url-badge]][url] | [![cat-encoding-badge]][cat-encoding] |
| [Encode and decode hex][ex-hex-encode-decode] | [![data-encoding-badge]][data-encoding] | [![cat-encoding-badge]][cat-encoding] |
| [Encode and decode base64][ex-base64] | [![base64-badge]][base64] | [![cat-encoding-badge]][cat-encoding] |
| [Read CSV records with different delimeter][ex-csv-delimiter] | [![csv-badge]][csv] | [![cat-encoding-badge]][cat-encoding] |

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
# #[macro_use]
# extern crate error_chain;
#[macro_use]
extern crate serde_json;

use serde_json::Value;
#
# error_chain! {
#     foreign_links {
#         Json(serde_json::Error);
#     }
# }

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
#
# quick_main!(run);
```

[ex-toml-config]: #ex-toml-config
<a name="ex-toml-config"></a>
## Deserialize a TOML configuration file

[![toml-badge]][toml] [![cat-encoding-badge]][cat-encoding]

Parse some TOML into a universal `toml::Value` that is able to represent any
valid TOML data.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate toml;

use toml::Value;
#
# error_chain! {
#     foreign_links {
#         Toml(toml::de::Error);
#     }
# }

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
    assert_eq!(package_info["package"]["name"].as_str(),
               Some("your_package"));

    Ok(())
}
#
# quick_main!(run);
```

Parse TOML into your own structs using Serde:

[![serde-json-badge]][serde-json] [![toml-badge]][toml] [![cat-encoding-badge]][cat-encoding]

```rust
# #[macro_use]
# extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;

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
#
# error_chain! {
#     foreign_links {
#         Toml(toml::de::Error);
#     }
# }

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
#
# quick_main!(run);
```

[ex-percent-encode]: #ex-percent-encode
<a name="ex-percent-encode"></a>
## Percent-encode a string

[![url-badge]][url] [![cat-encoding-badge]][cat-encoding]

Encode an input string with [percent-encoding] using the [`utf8_percent_encode`]
function from the `url` crate. Then decode using the [`percent_decode`]
function.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate url;

use url::percent_encoding::{utf8_percent_encode, percent_decode, DEFAULT_ENCODE_SET};
#
# error_chain! {
#     foreign_links {
#         Utf8(std::str::Utf8Error);
#     }
# }

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
#
# quick_main!(run);
```

The encode set defines which bytes (in addition to non-ASCII and controls) need
to be percent-encoded. The choice of this set depends on context. For example,
`?` needs to be encoded in a URL path but not in a query string.

The return value of encoding is an iterator of `&str` slices which can be
collected into a `String`.

[ex-urlencoded]: #ex-urlencoded
<a name="ex-urlencoded"></a>
## Encode a string as application/x-www-form-urlencoded

[![url-badge]][url] [![cat-encoding-badge]][cat-encoding]

Encodes a string into [application/x-www-form-urlencoded] syntax
using the [`form_urlencoded::byte_serialize`] and subsequently
decodes it with [`form_urlencoded::parse`]. Both functions return iterators
that can be collected into a `String`.

```rust
extern crate url;
use url::form_urlencoded::{byte_serialize, parse};

fn main() {
    let urlencoded: String = byte_serialize("What is ❤?".as_bytes()).collect();
    assert_eq!(urlencoded, "What+is+%E2%9D%A4%3F");
    println!("urlencoded:'{}'", urlencoded);

    let decoded: String = parse(urlencoded.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    assert_eq!(decoded, "What is ❤?");
    println!("decoded:'{}'", decoded);
}
```

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
# #[macro_use]
# extern crate error_chain;
extern crate data_encoding;

use data_encoding::{HEXUPPER, DecodeError};
#
# error_chain! {
#     foreign_links {
#         Decode(DecodeError);
#     }
# }

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
#
# quick_main!(run);
```

[ex-base64]: #ex-base64
<a name="ex-base64"></a>
## Encode and decode base64

[![base64-badge]][base64] [![cat-encoding-badge]][cat-encoding]

Encodes byte slice into `base64` String with help of [`encode`]
and subsequently decodes it with [`decode`].

[`decode`]: https://docs.rs/base64/*/base64/fn.decode.html
[`encode`]: https://docs.rs/base64/*/base64/fn.encode.html

```rust
# #[macro_use]
# extern crate error_chain;
extern crate base64;

use std::str;
use base64::{encode, decode};
#
# error_chain! {
#     foreign_links {
#         Base64(base64::DecodeError);
#         Utf8Error(str::Utf8Error);
#     }
# }

fn run() -> Result<()> {
    let hello = b"hello rustaceans";
    let encoded = encode(hello);
    let decoded = decode(&encoded)?;

    println!("origin: {}", str::from_utf8(hello)?);
    println!("base64 encoded: {}", encoded);
    println!("back to origin: {}", str::from_utf8(&decoded)?);

    Ok(())
}
#
# quick_main!(run);
```

[ex-csv-delimiter]: #ex-csv-delimiter
<a name="ex-csv-delimiter"></a>
## Read CSV records with different delimeter

[![csv-badge]][csv] [![cat-encoding-badge]][cat-encoding]

Reads CSV records with [`delimiter`] other than ','

[`delimiter`]: https://docs.rs/csv/1.0.0-beta.3/csv/struct.ReaderBuilder.html#method.delimiter

```rust
# #[macro_use]
# extern crate error_chain;
extern crate csv;

use csv::ReaderBuilder;
#
# error_chain! {
#     foreign_links {
#         CsvError(csv::Error);
#     }
# }

fn run() -> Result<()> {
    let data = "name-place-id
        Mark-Melbourne-46
        Ashley-Zurich-92";

    let mut reader = ReaderBuilder::new().delimiter(b'-').from_reader(data.as_bytes());
    for result in reader.records() {
        println!("{:?}", result?);
    }

    Ok(())
}
#
# quick_main!(run);
```

<!-- Categories -->

[cat-encoding-badge]: https://badge-cache.kominick.com/badge/encoding--x.svg?style=social
[cat-encoding]: https://crates.io/categories/encoding

<!-- Crates -->

[base64-badge]: https://badge-cache.kominick.com/crates/v/base64.svg?label=base64
[base64]: https://docs.rs/base64/
[data-encoding-badge]: https://badge-cache.kominick.com/crates/v/data-encoding.svg?label=data-encoding
[data-encoding]: https://github.com/ia0/data-encoding
[serde-json-badge]: https://badge-cache.kominick.com/crates/v/serde_json.svg?label=serde_json
[serde-json]: https://docs.serde.rs/serde_json/
[toml-badge]: https://badge-cache.kominick.com/crates/v/toml.svg?label=toml
[toml]: https://docs.rs/toml/
[url-badge]: https://badge-cache.kominick.com/crates/v/url.svg?label=url
[url]: https://docs.rs/url/
[csv-badge]: https://badge-cache.kominick.com/crates/v/csv.svg?label=csv
[csv]: https://docs.rs/csv/


<!-- Reference -->

[percent-encoding]: https://en.wikipedia.org/wiki/Percent-encoding
[`utf8_percent_encode`]: https://docs.rs/url/1.*/url/percent_encoding/fn.utf8_percent_encode.html
[`percent_decode`]: https://docs.rs/url/1.*/url/percent_encoding/fn.percent_decode.html
[application/x-www-form-urlencoded]: https://url.spec.whatwg.org/#application/x-www-form-urlencoded
[`form_urlencoded::byte_serialize`]: https://docs.rs/url/1.4.0/url/form_urlencoded/fn.byte_serialize.html
[`form_urlencoded::parse`]: https://docs.rs/url/*/url/form_urlencoded/fn.parse.html
