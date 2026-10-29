use std::io::{Read, Write};

use bzip2::Compression;
use bzip2::read::BzDecoder;
use bzip2::write::BzEncoder;

fn main() -> std::io::Result<()> {
    let data = b"hello, bzip2! ".repeat(100);

    let mut encoder = BzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&data)?;
    let compressed = encoder.finish()?;
    println!("{} bytes -> {} bytes", data.len(), compressed.len());

    let mut decompressed = Vec::new();
    BzDecoder::new(compressed.as_slice()).read_to_end(&mut decompressed)?;
    assert_eq!(decompressed, data);
    Ok(())
}
