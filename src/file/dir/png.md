## Find all png files recursively

[![glob-badge]][glob] [![cat-filesystem-badge]][cat-filesystem]

Recursively find all PNG files in the current directory.
In this case, the `**` pattern matches the current directory and all subdirectories.

Use the `**` pattern in any path portion. For example, `/media/**/*.png`
matches all PNGs in `media` and it's subdirectories.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate glob;

use glob::glob;
#
# error_chain! {
#     foreign_links {
#         Glob(glob::GlobError);
#         Pattern(glob::PatternError);
#     }
# }

fn run() -> Result<()> {
    for entry in glob("**/*.png")? {
        println!("{}", entry?.display());
    }

    Ok(())
}
#
# quick_main!(run);
```
