use std::io::{Read, Write};

use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;

fn main() -> std::io::Result<()> {
    let data = b"hello, gzip! ".repeat(100);

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data)?;
    let compressed = encoder.finish()?;
    println!("{} bytes -> {} bytes", data.len(), compressed.len());

    let mut decompressed = Vec::new();
    GzDecoder::new(compressed.as_slice()).read_to_end(&mut decompressed)?;
    assert_eq!(decompressed, data);
    Ok(())
}
