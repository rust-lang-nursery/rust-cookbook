use walkdir::WalkDir;

const REMOVED_TESTS: &[&str] = &[
    "./src/web/clients/requests/header.md",
    "./src/web/clients/api/rate-limited.md",
];

fn main() {
    #[cfg(feature = "test-rand")]
    {
        let rand_paths = vec![
            "./src/algorithms/randomness/rand.md",
            "./src/algorithms/randomness/rand-range.md",
            "./src/algorithms/randomness/rand-dist.md",
            "./src/algorithms/randomness/rand-custom.md",
            "./src/algorithms/randomness/rand-passwd.md",
            "./src/algorithms/randomness/rand-choose.md",
        ];
        skeptic::generate_doc_tests(&rand_paths[..]);
        return;
    }

    let paths = WalkDir::new("./src/").into_iter()
        // convert paths to Strings
        .map(|p| p.unwrap().path().to_str().unwrap().to_string())
        // only compile markdown files
        .filter(|p| p.ends_with(".md"))
        .filter(|p| !REMOVED_TESTS.contains(&p.as_ref()))
        .collect::<Vec<_>>();

    skeptic::generate_doc_tests(&paths[..]);
}
