extern crate skeptic;

use std::fs;

fn main() {
    let paths = fs::read_dir("./src/").unwrap()
        // convert paths to Strings
        .map(|p| p.unwrap().path().to_str().unwrap().to_string())
        // only compile markdown files
        .filter(|p| p.ends_with(".md"))
        .collect::<Vec<_>>();

    skeptic::generate_doc_tests(&paths[..]);
}
