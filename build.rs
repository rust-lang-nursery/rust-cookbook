use walkdir::WalkDir;

const REMOVED_TESTS: &[&str] = &[
    "./src/web/clients/requests/header.md",
    "./src/web/clients/api/rate-limited.md",
];

const REMOVED_PREFIXES: &[&str] = &[
    "./src/algorithms/randomness/",
    "./src/development_tools/debugging/tracing/",
];

fn main() {
    let paths = WalkDir::new("./src/")
        .into_iter()
        // convert paths to Strings
        .map(|p| p.unwrap().path().to_str().unwrap().to_string())
        // only compile markdown files
        .filter(|p| p.ends_with(".md"))
        .filter(|p| !REMOVED_TESTS.contains(&p.as_ref()))
        .filter(|p| !REMOVED_PREFIXES.iter().any(|prefix| p.starts_with(prefix)))
        .collect::<Vec<_>>();

    skeptic::generate_doc_tests(&paths[..]);
}
