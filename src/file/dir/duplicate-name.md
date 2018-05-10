## Recursively find duplicate file names

[![walkdir-badge]][walkdir] [![cat-filesystem-badge]][cat-filesystem]

Find recursively in the current directory duplicate filenames,
printing them only once.

```rust,no_run
extern crate walkdir;

use std::collections::HashMap;
use walkdir::WalkDir;

fn main() {
    // Counters indexed by filenames
    let mut filenames = HashMap::new();

    // List recusively all files in the current directory filtering out
    // directories and files not accessible (permission denied)
    for entry in WalkDir::new(".")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir()) {
        // Get entry's filename
        let f_name = String::from(entry.file_name().to_string_lossy());
        // Get or initialize the counter
        let counter = filenames.entry(f_name.clone()).or_insert(0);
        // Update the counter
        *counter += 1;

        if *counter == 2 {
            println!("{}", f_name);
        }
    }
}
```
