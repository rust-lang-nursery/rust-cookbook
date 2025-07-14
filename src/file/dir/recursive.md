```rust,edition2021
use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new(".")
                         .follow_links(true)
                         .into_iter()
                         .filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();
        let sec = entry.metadata().unwrap().modified().unwrap();

        if f_name.ends_with(".json") && sec.elapsed().unwrap().as_secs() < 86400 {
            println!("{}", entry.path().display());
        }
    }
}
``` 