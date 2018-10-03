## Make a partial download with HTTP range headers

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

Uses [`reqwest::Client::head`] to get the [Content-Length] of the response.

The code then uses [`reqwest::Client::get`] to download the content in
chunks of 10240 bytes, while printing progress messages. The [Range] header specifies the chunk size and position.

The Range header is defined in [RFC7233][HTTP Range RFC7233].

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate reqwest;

use std::fs::File;
use std::str::FromStr;
use reqwest::header::{HeaderValue, CONTENT_LENGTH, RANGE};
use reqwest::StatusCode;

#
# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#         Reqwest(reqwest::Error);
#         Header(reqwest::header::ToStrError);
#     }
# }
#
# struct PartialRangeIter {
#     start: u64,
#     end: u64,
#     buffer_size: u32,
# }
#
# impl PartialRangeIter {
#     pub fn new(start: u64, end: u64, buffer_size: u32) -> Result<Self> {
#         if buffer_size == 0 {
#             Err("invalid buffer_size, give a value greater than zero.")?;
#         }
#
#         Ok(PartialRangeIter {
#             start,
#             end,
#             buffer_size,
#         })
#     }
# }
#
# impl Iterator for PartialRangeIter {
#     type Item = HeaderValue;
#
#     fn next(&mut self) -> Option<Self::Item> {
#         if self.start > self.end {
#             None
#         } else {
#             let prev_start = self.start;
#             self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);
#             // NOTE(unwrap): `HeaderValue::from_str` will fail only if the value is not made
#             // of visible ASCII characters. Since the format string is static and the two
#             // values are integers, that can't happen.
#             Some(HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1)).unwrap())
#         }
#     }
# }

fn run() -> Result<()> {
    let url = "https://httpbin.org/range/102400?duration=2";
    const CHUNK_SIZE: u32 = 10240;

    let client = reqwest::Client::new();
    let response = client.head(url).send()?;
    let length = response
        .headers()
        .get(CONTENT_LENGTH)
        .ok_or("response doesn't include the content length")?;
    let length = u64::from_str(length.to_str()?).map_err(|_| "invalid Content-Length header")?;

    let mut output_file = File::create("download.bin")?;

    println!("starting download...");
    for range in PartialRangeIter::new(0, length - 1, CHUNK_SIZE)? {
        println!("range {:?}", range);
        let mut response = client.get(url).header(RANGE, range).send()?;

        let status = response.status();
        if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
            bail!("Unexpected server response: {}", status)
        }

        std::io::copy(&mut response, &mut output_file)?;
    }

    println!("Finished with success!");
    Ok(())
}
#
# quick_main!(run);
```

[`reqwest::Client::get`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.get
[`reqwest::Client::head`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.head
[Content-Length]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Length
[Range]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Range

[HTTP Range RFC7233]: https://tools.ietf.org/html/rfc7233#section-3.1
