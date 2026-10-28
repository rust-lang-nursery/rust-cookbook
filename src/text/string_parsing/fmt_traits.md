## Implement Debug, Display, and Hash for a custom type

[![std-badge]][std] [![cat-text-processing-badge]][cat-text-processing]

Manually implements the [`Debug`] and [`Display`] traits for a custom
struct with mixed field types, and derives [`Hash`] for it. A manual
`Debug` implementation is useful when the default output produced by
`#[derive(Debug)]` isn't quite what's needed; [`Formatter::debug_struct`]
builds output in that same `Field { .. }` style without having to format
the fields by hand.

```rust,edition2024
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Hash)]
struct Package {
    name: String,
    version: (u8, u8, u8),
    downloads: u64,
}

impl fmt::Debug for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Package")
            .field("name", &self.name)
            .field("version", &self.version)
            .field("downloads", &self.downloads)
            .finish()
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (major, minor, patch) = self.version;
        write!(
            f,
            "{} v{}.{}.{} ({} downloads)",
            self.name, major, minor, patch, self.downloads
        )
    }
}

fn main() {
    let package = Package {
        name: "rust-cookbook".to_string(),
        version: (1, 0, 0),
        downloads: 42_017,
    };

    println!("Debug:   {:?}", package);
    println!("Display: {}", package);

    let mut hasher = DefaultHasher::new();
    package.hash(&mut hasher);
    println!("Hash:    {:x}", hasher.finish());
}
```

[`Debug`]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
[`Formatter::debug_struct`]: https://doc.rust-lang.org/std/fmt/struct.Formatter.html#method.debug_struct
