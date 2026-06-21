## Transform an Option value

[![std-badge]][std] [![cat-rust-patterns-badge]][cat-rust-patterns]

Rather than matching on every [`Option`] by hand, the standard library
provides combinators that transform the contained value in place. This recipe
classifies an uploaded file by extension: [`Option::and_then`] chains
[`Path::extension`] into [`OsStr::to_str`], where each step may itself yield
`None`; [`Option::map`] normalizes the matched value; and [`Option::filter`]
rejects anything that is not a supported image. `None` falls straight through
the chain, so no intermediate value needs to be unwrapped.

```rust,edition2021
use std::path::Path;

/// Return the lowercase extension of `filename`, but only when it names an
/// image we are willing to accept.
fn image_extension(filename: &str) -> Option<String> {
    Path::new(filename)
        .extension() // `Option<&OsStr>`: `None` when there is no extension
        .and_then(|ext| ext.to_str()) // `and_then` chains another `Option`-returning step
        .map(|ext| ext.to_lowercase()) // `map` normalizes the matched value
        .filter(|ext| matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "gif"))
}

fn main() {
    println!("photo.JPG      -> {:?}", image_extension("photo.JPG"));
    println!("archive.tar.gz -> {:?}", image_extension("archive.tar.gz"));
    println!("README         -> {:?}", image_extension("README"));

    assert_eq!(image_extension("photo.JPG"), Some("jpg".to_string()));
    assert_eq!(image_extension("archive.tar.gz"), None);
    assert_eq!(image_extension("README"), None);
}
```
