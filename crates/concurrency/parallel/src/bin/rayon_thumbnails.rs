use anyhow::Result;
use std::fs::create_dir_all;
use std::path::Path;

use glob::{MatchOptions, glob_with};
use image::imageops::FilterType;
use rayon::prelude::*;

fn main() -> Result<()> {
    let options: MatchOptions = Default::default();
    let files: Vec<_> = glob_with("*.jpg", options)?
        .filter_map(|x| x.ok())
        .collect();

    if files.is_empty() {
        anyhow::bail!("No .jpg files found in current directory");
    }

    let thumb_dir = "thumbnails";
    create_dir_all(thumb_dir)?;

    println!("Saving {} thumbnails into '{}'...", files.len(), thumb_dir);

    let image_failures: Vec<_> = files
        .par_iter()
        .map(|path| {
            make_thumbnail(path, thumb_dir, 300)
                .map_err(|e| anyhow::anyhow!("Failed to process {}: {}", path.display(), e))
        })
        .filter_map(|x| x.err())
        .collect();

    image_failures.iter().for_each(|x| println!("{}", x));

    println!(
        "{} thumbnails saved successfully",
        files.len() - image_failures.len()
    );
    Ok(())
}

fn make_thumbnail<PA, PB>(original: PA, thumb_dir: PB, longest_edge: u32) -> Result<()>
where
    PA: AsRef<Path>,
    PB: AsRef<Path>,
{
    let img = image::open(original.as_ref())?;
    let file_path = thumb_dir.as_ref().join(original);

    Ok(img
        .resize(longest_edge, longest_edge, FilterType::Nearest)
        .save(file_path)?)
}
