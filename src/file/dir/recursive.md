```rust,edition2018
extern crate walkdir;
use walkdir::WalkDir;

fn main() {
    // Example of using WalkDir to recursively traverse directories
    for entry in WalkDir::new(".").max_depth(2) {
        match entry {
            Ok(entry) => println!("{}", entry.path().display()),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
``` 