## Recursively find duplicate file names

[![walkdir-badge]][walkdir] [![cat-filesystem-badge]][cat-filesystem]

Find recursively in the current directory duplicate filenames,
printing them only once.

```rust,edition2021
use walkdir::WalkDir;
use std::collections::HashMap;

fn main() {
    let mut filenames = HashMap::new();

    for entry in WalkDir::new(".")
                         .into_iter()
                         .filter_map(Result::ok)
                         .filter(|e| e.file_type().is_file()) {

        let f_name = String::from(entry.file_name().to_string_lossy());
        let counter = filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;

        if *counter == 2 {
            println!("{}", f_name);
        }
    }
}
