## Download and read a gzip file

[![std-badge]][std] [![flate2-badge]][flate2] [![reqwest-badge]][reqwest] [![cat-compression-badge]][cat-compression]

Download a sample `gzip` file and read it.

```rust,no_run
extern crate flate2;
extern crate reqwest;

use std::error;
use std::io::{copy, Read};
use flate2::bufread::GzDecoder;
use reqwest::get;

fn main() -> Result<(), Box<error::Error>> {

    // Download an example of a compressed json file
    let mut response = get("https://wiki.mozilla.org/images/f/ff/Example.json.gz")?;

    // Store the downloaded file in a buffer
    // Note: This consumes the body of `response`
    let mut buf: Vec<u8> = Vec::new ();
    copy (&mut response, &mut buf)?;

    // Decode the downloaded file
    let mut gz = GzDecoder::new (&buf [..]);
    let mut json = String::new ();
    gz.read_to_string(&mut json)?;

    println!("{}", json);
    
    Ok (())
}
```
