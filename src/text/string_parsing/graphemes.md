## Collect Unicode Graphemes

[![unicode-segmentation-badge]][`unicode-segmentation`] [![cat-text-processing-badge]][cat-text-processing]

Collect individual Unicode graphemes from UTF-8 string using the 
[`UnicodeSegmentation::graphemes`] function from the [`unicode-segmentation`] crate.

```rust
#[macro_use]
extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let name = "José Guimarães\r\n";
    let graphemes = UnicodeSegmentation::graphemes(name, true)
    	.collect::<Vec<&str>>();
	assert_eq!(graphemes[3], "é");
}
```

[`UnicodeSegmentation::graphemes`]: https://docs.rs/unicode-segmentation/*/unicode_segmentation/trait.UnicodeSegmentation.html#tymethod.graphemes
[`unicode-segmentation`]: https://docs.rs/unicode-segmentation/1.2.1/unicode_segmentation/
