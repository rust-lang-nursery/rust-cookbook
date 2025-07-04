## Generate jpg thumbnails in parallel

[![rayon-badge]][rayon] [![glob-badge]][glob] [![image-badge]][image] [![cat-concurrency-badge]][cat-concurrency] [![cat-filesystem-badge]][cat-filesystem]

This example generates thumbnails for all .jpg files in the current directory
then saves them in a new folder called `thumbnails`.

[`glob::glob_with`] finds jpeg files in current directory. `rayon` resizes
images in parallel using [`par_iter`] calling  [`DynamicImage::resize`].

```rust,edition2018,no_run
extern crate rayon;
extern crate image;
extern crate glob;
extern crate anyhow;
use rayon::prelude::*;
use anyhow::{Result, anyhow, Context};

use std::path::Path;
use std::fs::create_dir_all;
use glob::{glob_with, MatchOptions};
use image::{FilterType, ImageError};

fn main() -> Result<()> {
    let options: MatchOptions = Default::default();
    let files: Vec<_> = glob_with("*.jpg", options)?
        .filter_map(|x| x.ok())
        .collect();

    if files.len() == 0 {
        return Err(anyhow!("No .jpg files found in current directory"));
    }

    let thumb_dir = "thumbnails";
    create_dir_all(thumb_dir)?;

    println!("Saving {} thumbnails into '{}'...", files.len(), thumb_dir);

    let image_failures: Vec<_> = files
        .par_iter()
        .map(|path| {
            make_thumbnail(path, thumb_dir, 300)
                .with_context(|| format!("Failed to process {}", path.display()))
        })
        .filter_map(|x| x.err())
        .collect();

    image_failures.iter().for_each(|x| println!("{}", x));

    println!("{} thumbnails saved successfully", files.len() - image_failures.len());
    Ok(())
}

fn make_thumbnail<PA, PB>(original: PA, thumb_dir: PB, longest_edge: u32) -> Result<()>
where
    PA: AsRef<Path>,
    PB: AsRef<Path>,
{
    let img = image::open(original.as_ref())?;
    let file_path = thumb_dir.as_ref().join(original);

    Ok(img.resize(longest_edge, longest_edge, FilterType::Nearest)
        .save(file_path)?)
}
```

[`glob::glob_with`]: https://docs.rs/glob/*/glob/fn.glob_with.html
[`par_iter`]: https://docs.rs/rayon/*/rayon/iter/trait.IntoParallelRefIterator.html#tymethod.par_iter
[`DynamicImage::resize`]: https://docs.rs/image/*/image/enum.DynamicImage.html#method.resize
