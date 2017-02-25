extern crate skeptic;

use std::fs;

fn main() {
    let paths = fs::read_dir("./pages/").unwrap();
    let mut vec = Vec::new();

    // Convert paths to strings and add to vec
    for path in paths {
        let p = path.unwrap().path();
        let s = p.to_str().unwrap();
        if s.contains(".md") {
            vec.push(s.to_string());
        }
    }
    // Convert Vec<String> to Vec<str>
    let vec: Vec<_> = vec.iter().map(|s|&s[..]).collect();
    skeptic::generate_doc_tests(&vec);
}
