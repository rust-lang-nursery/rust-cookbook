#[cfg(test)]
mod book_check;
mod chips;
mod search_index;

use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(e) = chips::run_preprocess(&args) {
        eprintln!("mdbook-chips: {e}");
        std::process::exit(1);
    }
}

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
        .to_path_buf()
}
