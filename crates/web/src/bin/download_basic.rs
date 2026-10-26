use anyhow::Result;
use std::fs::File;
use std::io::Write;
use tempfile::Builder;

fn main() -> Result<()> {
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response = reqwest::blocking::get(target)?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|mut segments| segments.next_back())
            .filter(|name| !name.is_empty())
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let fname = tmp_dir.path().join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname)?
    };
    let content = response.bytes()?;
    dest.write_all(&content)?;
    Ok(())
}
