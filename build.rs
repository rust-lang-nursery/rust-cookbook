extern crate skeptic;
extern crate walkdir;

use walkdir::WalkDir;

fn main() {
    let paths = WalkDir::new("./src/").into_iter()
        // convert paths to Strings
        .map(|p| p.unwrap().path().to_str().unwrap().to_string())
        // only compile markdown files
        .filter(|p| p.ends_with(".md"))
        .collect::<Vec<_>>();

    skeptic::generate_doc_tests(&paths[..]);
}
