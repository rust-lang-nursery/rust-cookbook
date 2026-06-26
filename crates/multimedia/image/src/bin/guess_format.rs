use std::path::Path;

use anyhow::Result;
use image::ImageFormat;

fn detect(bytes: &[u8]) -> Result<ImageFormat> {
    Ok(image::guess_format(bytes)?)
}

fn main() -> Result<()> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/sample.png");
    let bytes = std::fs::read(&path)?;

    let format = detect(&bytes)?;
    println!("{:?} ({})", format, format.to_mime_type());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_png_from_its_header() -> Result<()> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/sample.png");
        let bytes = std::fs::read(&path)?;
        assert_eq!(detect(&bytes)?, ImageFormat::Png);
        Ok(())
    }

    #[test]
    fn rejects_data_that_is_not_an_image() {
        assert!(detect(b"not an image").is_err());
    }
}
