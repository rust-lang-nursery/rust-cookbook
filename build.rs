use walkdir::WalkDir;

const REMOVED_TESTS: &[&str] = &[
    "./src/about.md",
    "./src/file/watch.md",
    "./src/file/watch/recursive.md",
    "./src/file/watch/debounce.md",
    "./src/file/which.md",
    "./src/file/read-write/memmap.md",
    "./src/concurrency/parallel/rayon-parallel-sort.md",
    "./src/web/leptos.md",
    "./src/wasm.md",
    "./src/data_structures/bitfield/bitfield.md",
    "./src/concurrency/parallel/rayon-thumbnails.md",
    "./src/concurrency/thread/threadpool-fractal.md",
    "./src/encoding/complex/toml.md",
    "./src/encoding/string/base32.md",
    "./src/encoding/string/base64.md",
    "./src/web/mime/request.md",
    "./src/errors/handle/retain.md",
];

const REMOVED_PREFIXES: &[&str] = &[
    "./src/algorithms/randomness/",
    "./src/compression/bzip2/",
    "./src/cli/ansi_terminal/",
    "./src/configuration/",
    "./src/development_tools/build_tools/",
    "./src/development_tools/debugging/config_log/log-custom.md",
    "./src/development_tools/debugging/log/log-syslog.md",
    "./src/development_tools/debugging/tracing/",
    "./src/multimedia/",
    "./src/parsing/",
    "./src/concurrency/actor/",
    "./src/concurrency/custom_future/",
    "./src/safety_critical/no_panic/",
    "./src/safety_critical/heapless_alloc/",
    "./src/science/mathematics/linear_algebra/",
    "./src/wasm/",
    "./src/web/clients/",
    "./src/web/scraping/",
    "./src/datetime/jiff/",
    "./src/datetime/time/",
    "./src/cryptography/hashing/rustcrypto-",
    "./src/cryptography/rustcrypto/",
    "./src/database/sqlite/",
    "./src/database/sqlx/",
    "./src/database/sea_orm/",
    "./src/encoding/rkyv",
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
