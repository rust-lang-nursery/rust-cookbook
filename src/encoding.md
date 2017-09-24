# Encoding

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Serialize and deserialize unstructured JSON][ex-json-value] | [![serde-json-badge]][serde-json] | [![cat-encoding-badge]][cat-encoding] |
| [Deserialize a TOML configuration file][ex-toml-config] | [![toml-badge]][toml] | [![cat-encoding-badge]][cat-encoding] |
| [Percent-encode a string][ex-percent-encode] | [![url-badge]][url] | [![cat-encoding-badge]][cat-encoding] |
| [Encode a string as application/x-www-form-urlencoded][ex-urlencoded] | [![url-badge]][url] | [![cat-encoding-badge]][cat-encoding] |
| [Encode and decode hex][ex-hex-encode-decode] | [![data-encoding-badge]][data-encoding] | [![cat-encoding-badge]][cat-encoding] |
| [Encode and decode base64][ex-base64] | [![base64-badge]][base64] | [![cat-encoding-badge]][cat-encoding] |
| [Serialize records to CSV][ex-serialize-csv] | [![csv-badge]][csv] | [![cat-encoding-badge]][cat-encoding] |
| [Serialize records to CSV using Serde][ex-csv-serde] | [![csv-badge]][csv] [![serde-badge]][serde] | [![cat-encoding-badge]][cat-encoding] |
| [Handle invalid CSV data with Serde][ex-invalid-csv] | [![csv-badge]][csv] [![serde-badge]][serde] | [![cat-encoding-badge]][cat-encoding] |
| [Read CSV records with different delimeter][ex-csv-delimiter] | [![csv-badge]][csv] | [![cat-encoding-badge]][cat-encoding] |
| [Filter CSV records matching a predicate][ex-csv-filter] | [![csv-badge]][csv] | [![cat-encoding-badge]][cat-encoding] |

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

[ex-serialize-csv]: #ex-serialize-csv
<a name="ex-serialize-csv"></a>
## Serialize records to CSV

[![csv-badge]][csv] [![cat-encoding-badge]][cat-encoding]

This example shows how to serialize a Rust tuple. [`csv::writer`] supports automatic
serialization from Rust types into CSV records. [`write_record`] is used when writing
a simple record that contains string-like data only, [`serialize`] is used when data
consists of more complex values like numbers, floats or optional values. Since CSV
writer uses internal buffer, always explicitly [`flush`] when done.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate csv;

use std::io;
#
# error_chain! {
#     foreign_links {
#         CSVError(csv::Error);
#         IOError(std::io::Error);
#    }
# }

fn run() -> Result<()> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(&["Name", "Place", "ID"])?;

    wtr.serialize(("Mark", "Sydney", 87))?;
    wtr.serialize(("Ashley", "Dublin", 32))?;
    wtr.serialize(("Akshat", "Delhi", 11))?;

    wtr.flush()?;
    Ok(())
}
#
# quick_main!(run);
```

[ex-csv-serde]: #ex-csv-serde
<a name="ex-csv-serde"></a>
## Serialize records to CSV using Serde

[![csv-badge]][csv] [![serde-badge]][serde] [![cat-encoding-badge]][cat-encoding]

The following example shows how to serialize custom structs as CSV records using
the [serde] crate.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate csv;
#[macro_use]
extern crate serde_derive;

use std::io;
#
# error_chain! {
#    foreign_links {
#        IOError(std::io::Error);
#        CSVError(csv::Error);
#    }
# }

#[derive(Debug, Serialize)]
struct Record<'a> {
    name: &'a str,
    place: &'a str,
    id: u64,
}

fn run() -> Result<()> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    let rec1 = Record { name: "Mark", place: "Melbourne", id: 56};
    let rec2 = Record { name: "Ashley", place: "Sydney", id: 64};
    let rec3 = Record { name: "Akshat", place: "Delhi", id: 98};

    wtr.serialize(rec1)?;
    wtr.serialize(rec2)?;
    wtr.serialize(rec3)?;

    wtr.flush()?;

    Ok(())
}
#
# quick_main!(run);
```

[ex-invalid-csv]: #ex-invalid-csv
<a name="ex-invalid-csv"></a>
## Handle invalid CSV data with Serde

[![csv-badge]][csv] [![serde-badge]][serde] [![cat-encoding-badge]][cat-encoding]

CSV files often contain invalid data. For these cases, the csv crate
provides a custom deserializer, [`csv::invalid_option`], which automatically
converts invalid data to None values.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate csv;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}
#
# error_chain! {
#     foreign_links {
#         CsvError(csv::Error);
#     }
# }

fn run() -> Result<()> {
    let data = "name,place,id
mark,sydney,46.5
ashley,zurich,92
akshat,delhi,37
alisha,colombo,xyz";

    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }

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
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

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

[ex-csv-filter]: #ex-csv-filter
<a name="ex-csv-filter"></a>
## Filter CSV records matching a predicate

[![csv-badge]][csv] [![cat-encoding-badge]][cat-encoding]


Returns _only_ the rows from `data` with a field that matches `query`.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate csv;

use std::io;
#
# error_chain!{
#     foreign_links {
#         Io(std::io::Error);
#         CsvError(csv::Error); // or just Seek(csv::Error)
#     }
# }

fn run() -> Result<()> {
    let query = "CA";
    let data = "\
City,State,Population,Latitude,Longitude
Kenai,AK,7610,60.5544444,-151.2583333
Oakman,AL,,33.7133333,-87.3886111
Sandfort,AL,,32.3380556,-85.2233333
West Hollywood,CA,37031,34.0900000,-118.3608333";

    let mut rdr = csv::ReaderBuilder::new().from_reader(data.as_bytes());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(rdr.headers()?)?;

    for result in rdr.records() {
        let record = result?;
        if record.iter().any(|field| field == query) {
            wtr.write_record(&record)?;
        }
    }

    wtr.flush()?;
    Ok(())
}
#
# quick_main!(run);
```

_Disclaimer: this example has been adapted from [the csv crate tutorial](https://docs.rs/csv/*/csv/tutorial/index.html#filter-by-search)_.

{{#include links.md}}

<!-- API Reference -->

[`csv::Writer`]: https://docs.rs/csv/*/csv/struct.Writer.html
[`csv::invalid_option`]: https://docs.rs/csv/*/csv/fn.invalid_option.html
[`flush`]: https://docs.rs/csv/*/csv/struct.Writer.html#method.flush
[`form_urlencoded::byte_serialize`]: https://docs.rs/url/1.4.0/url/form_urlencoded/fn.byte_serialize.html
[`form_urlencoded::parse`]: https://docs.rs/url/*/url/form_urlencoded/fn.parse.html
[`percent_decode`]: https://docs.rs/url/1.*/url/percent_encoding/fn.percent_decode.html
[`serialize`]: https://docs.rs/csv/*/csv/struct.Writer.html#method.serialize
[`utf8_percent_encode`]: https://docs.rs/url/1.*/url/percent_encoding/fn.utf8_percent_encode.html
[`write_record`]: https://docs.rs/csv/*/csv/struct.Writer.html#method.write_record

<!-- Other Reference -->

[application/x-www-form-urlencoded]: https://url.spec.whatwg.org/#application/x-www-form-urlencoded
[percent-encoding]: https://en.wikipedia.org/wiki/Percent-encoding
