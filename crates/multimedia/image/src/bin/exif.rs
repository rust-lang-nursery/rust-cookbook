use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use exif::{Exif, In, Reader, Tag};

fn read_exif(path: &Path) -> Result<Exif> {
    let mut reader = BufReader::new(File::open(path)?);
    Ok(Reader::new().read_from_container(&mut reader)?)
}

fn main() -> Result<()> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/photo.jpg");
    let exif = read_exif(&path)?;

    for tag in [Tag::Make, Tag::Model, Tag::Software, Tag::DateTime, Tag::Orientation] {
        if let Some(field) = exif.get_field(tag, In::PRIMARY) {
            println!("{}: {}", tag, field.display_value().with_unit(&exif));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_camera_make() -> Result<()> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/photo.jpg");
        let exif = read_exif(&path)?;

        let make = exif
            .get_field(Tag::Make, In::PRIMARY)
            .map(|f| f.display_value().to_string());
        assert_eq!(make.as_deref(), Some("\"Cookbook Camera Co.\""));
        Ok(())
    }
}
