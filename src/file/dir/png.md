## Find all png files recursively

[![glob-badge]][glob] [![cat-filesystem-badge]][cat-filesystem]

Recursively find all PNG files in the current directory.
In this case, the `**` pattern matches the current directory and all subdirectories.

Use the `**` pattern in any path portion. For example, `/media/**/*.png`
matches all PNGs in `media` and it's subdirectories.

```rust,edition2018,no_run
# use error_chain::error_chain;

use glob::glob;
#
# error_chain! {
#     foreign_links {
#         Glob(glob::GlobError);
#         Pattern(glob::PatternError);
#     }
# }

fn main() -> Result<()> {
    for entry in glob("**/*.png")? {
        println!("{}", entry?.display());
    }

    Ok(())
}
```
