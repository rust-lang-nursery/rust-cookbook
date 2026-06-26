use std::path::Path;

use anyhow::Result;
use image::DynamicImage;
use image::imageops::FilterType;

fn thumbnail(source: &Path, max_edge: u32) -> Result<DynamicImage> {
    let image = image::open(source)?;
    Ok(image.resize(max_edge, max_edge, FilterType::Lanczos3))
}

fn main() -> Result<()> {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/sample.png");
    let thumb = thumbnail(&source, 64)?;

    let out = std::env::temp_dir().join("thumbnail.jpg");
    thumb.save(&out)?;

    println!(
        "wrote {}x{} thumbnail to {}",
        thumb.width(),
        thumb.height(),
        out.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/sample.png")
    }

    #[test]
    fn fits_within_the_bounding_box() -> Result<()> {
        let thumb = thumbnail(&sample(), 64)?;
        assert!(thumb.width() <= 64 && thumb.height() <= 64);
        Ok(())
    }

    #[test]
    fn re_encodes_as_jpeg() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let out = dir.path().join("thumb.jpg");
        thumbnail(&sample(), 64)?.save(&out)?;

        let bytes = std::fs::read(&out)?;
        assert_eq!(image::guess_format(&bytes)?, image::ImageFormat::Jpeg);
        Ok(())
    }
}
