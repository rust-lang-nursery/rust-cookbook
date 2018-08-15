## Decompress gzip response for a http request

[![flate2-badge]][flate2] [![reqwest-badge]][reqwest] [![cat-decompression-badge]][cat-decompression]

Donwload a content with [`reqwest`]. </br>
[`GzEncoder`] is responsible for decompress the response.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate reqwest;
extern crate flate2;
#
# error_chain! {
#     foreign_links {
#         ReqError(reqwest::Error);
#         Io(std::io::Error);
#     }
# }
use std::io::Read;
use flate2::read::GzDecoder;

fn main() -> Result<()> {
    let response = reqwest::get("https://httpbin.org/gzip")?;
    let mut d = GzDecoder::new(response);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    println!("{}", s);
    Ok(())
}
#
```

[`reqwest`]: https://docs.rs/reqwest
[`GzEncoder`]: https://docs.rs/flate2/*/flate2/write/struct.GzEncoder.html