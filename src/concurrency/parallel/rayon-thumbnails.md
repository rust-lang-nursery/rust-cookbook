[ex-rayon-thumbnails]: #ex-rayon-thumbnails
<a name="ex-rayon-thumbnails"></a>
## Generate jpg thumbnails in parallel

[![rayon-badge]][rayon] [![glob-badge]][glob] [![image-badge]][image] [![cat-concurrency-badge]][cat-concurrency] [![cat-filesystem-badge]][cat-filesystem]

This example generates thumbnails for all .jpg in the current directory and saves them in a new folder called `thumbnails`.

Files are found using [`glob::glob_with`] to match case insensitively on both `.jpg` and `.JPG`. `rayon` is then used to resize images in parallel using [`par_iter`] along with the `make_thumbnail()` helper function which internally uses [`DynamicImage::resize`].

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate glob;
extern crate image;
extern crate rayon;

use std::path::Path;
use std::fs::{create_dir_all, File};

# use error_chain::ChainedError;
use glob::{glob_with, MatchOptions};
use image::{FilterType, ImageError};
use rayon::prelude::*;

# error_chain! {
#     foreign_links {
#         Image(ImageError);
#         Io(std::io::Error);
#         Glob(glob::PatternError);
#     }
# }

fn run() -> Result<()> {
    // find all files in current directory that have a .jpg extension
    // use the default MatchOptions so the search is case insensitive
    let options: MatchOptions = Default::default();
    let files: Vec<_> = glob_with("*.jpg", &options)?
        .filter_map(|x| x.ok())
        .collect();

    if files.len() == 0 {
        bail!("No .jpg files found in current directory");
    }

    let thumb_dir = "thumbnails";
    create_dir_all(thumb_dir)?;

    println!("Saving {} thumbnails into '{}'...", files.len(), thumb_dir);

    let image_failures: Vec<_> = files
        .par_iter()
        .map(|path| {
            make_thumbnail(path, thumb_dir, 300)
                .map_err(|e| e.chain_err(|| path.display().to_string()))
        })
        .filter_map(|x| x.err())
        .collect();

    image_failures.iter().for_each(|x| println!("{}", x.display_chain()));

    println!("{} thumbnails saved successfully", files.len() - image_failures.len());
    Ok(())
}

/// Resize `original` to have a maximum dimension of `longest_edge` and save the
/// resized image to the `thumb_dir` folder
fn make_thumbnail<PA, PB>(original: PA, thumb_dir: PB, longest_edge: u32) -> Result<()>
where
    PA: AsRef<Path>,
    PB: AsRef<Path>,
{
    let img = image::open(original.as_ref())?;
    let fout = &mut File::create(thumb_dir.as_ref().join(original))?;

    Ok(img.resize(longest_edge, longest_edge, FilterType::Nearest)
        .save(fout, image::JPEG)?)
}
#
# quick_main!(run);
```

[`glob::glob_with`]: https://docs.rs/glob/*/glob/fn.glob_with.html
[`par_iter`]: https://docs.rs/rayon/*/rayon/iter/trait.IntoParallelRefIterator.html#tymethod.par_iter
[`DynamicImage::resize`]: https://docs.rs/image/*/image/enum.DynamicImage.html#method.resize
