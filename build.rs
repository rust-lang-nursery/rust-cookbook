extern crate skeptic;

use std::fs;

fn main() {
    let paths = fs::read_dir("./pages/").unwrap();

    for path in paths {
        let p = path.unwrap().path();
        let s = p.to_str().unwrap();
        if s.contains(".md") {
            skeptic::generate_doc_tests(&[s])
        }
    }
}
