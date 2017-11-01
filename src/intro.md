# Cookin' with Rust

This _Rust Cookbook_ is a collection of
simple examples that demonstrate good practices to accomplish common
programming tasks, using the crates of the Rust ecosystem.

[Read more about _Rust Cookbook_](about.html), including tips for
how to read the book, how to use the examples, and notes on conventions.

## Contributing

This project is intended to be easy for new Rust programmers to
contribute to, and an easy to way get involved with the Rust
community. It needs and welcomes help. For details see
[CONTRIBUTING.md].

[CONTRIBUTING.md]: https://github.com/rust-lang-nursery/rust-cookbook/blob/master/CONTRIBUTING.md

## [Basics](basics.html)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Read lines of strings from a file][ex-std-read-lines] | [![std-badge]][std] | [![cat-filesystem-badge]][cat-filesystem] |
| [Read and write integers in little-endian byte order][ex-byteorder-le] | [![byteorder-badge]][byteorder] | [![cat-encoding-badge]][cat-encoding] |
| [Generate random numbers][ex-rand] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random numbers within a range][ex-rand-range] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random numbers with given distribution][ex-rand-dist] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random values of a custom type][ex-rand-custom] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Run an external command and process stdout][ex-parse-subprocess-output] | [![regex-badge]][regex] | [![cat-os-badge]][cat-os] [![cat-text-processing-badge]][cat-text-processing] |
| [Run an external command passing it stdin and check for an error code][ex-parse-subprocess-input] | [![std-badge]][std] | [![cat-os-badge]][cat-os] |
| [Run piped external commands][ex-run-piped-external-commands] | [![std-badge]][std] | [![cat-os-badge]][cat-os] |
| [Redirect both stdout and stderr of child process to the same file][ex-redirect-stdout-stderr-same-file] | [![std-badge]][std] | [![cat-os-badge]][cat-os] |
| [Filter a log file by matching multiple regular expressions][ex-regex-filter-log] | [![regex-badge]][regex] | [![cat-text-processing-badge]][cat-text-processing]
| [Declare lazily evaluated constant][ex-lazy-constant] | [![lazy_static-badge]][lazy_static] | [![cat-caching-badge]][cat-caching] [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Maintain global mutable state][ex-global-mut-state] | [![lazy_static-badge]][lazy_static] | [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Verify and extract login from an email address][ex-verify-extract-email] | [![regex-badge]][regex] [![lazy_static-badge]][lazy_static] | [![cat-text-processing-badge]][cat-text-processing] |
| [Extract a list of unique #Hashtags from a text][ex-extract-hashtags] | [![regex-badge]][regex] [![lazy_static-badge]][lazy_static] | [![cat-text-processing-badge]][cat-text-processing] |
| [Replace all occurrences of one text pattern with another pattern.][ex-regex-replace-named] | [![regex-badge]][regex] [![lazy_static-badge]][lazy_static] | [![cat-text-processing-badge]][cat-text-processing] |
| [Extract phone numbers from text][ex-phone] | [![regex-badge]][regex] | [![cat-text-processing-badge]][cat-text-processing] |
| [Calculate the SHA-256 digest of a file][ex-sha-digest] | [![ring-badge]][ring] [![data-encoding-badge]][data-encoding] | [![cat-cryptography-badge]][cat-cryptography] |
| [Sign and verify a message with an HMAC digest][ex-hmac] | [![ring-badge]][ring] | [![cat-cryptography-badge]][cat-cryptography] |
| [Salt and hash a password with PBKDF2][ex-pbkdf2] | [![ring-badge]][ring] [![data-encoding-badge]][data-encoding] | [![cat-cryptography-badge]][cat-cryptography] |
| [Define and operate on a type represented as a bitfield][ex-bitflags] | [![bitflags-badge]][bitflags] | [![cat-no-std-badge]][cat-no-std] |
| [Access a file randomly using a memory map][ex-random-file-access] | [![memmap-badge]][memmap] | [![cat-filesystem-badge]][cat-filesystem] |
| [Check number of logical cpu cores][ex-check-cpu-cores] | [![num_cpus-badge]][num_cpus] | [![cat-hardware-support-badge]][cat-hardware-support] |
| [Obtain backtrace of complex error scenarios][ex-error-chain-backtrace] | [![error-chain-badge]][error-chain] | [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Measure elapsed time][ex-measure-elapsed-time] | [![std-badge]][std] | [![cat-time-badge]][cat-time] |
| [Display formatted date and time][ex-format-datetime] | [![chrono-badge]][chrono] | [![cat-date-and-time-badge]][cat-date-and-time] |

## [Encoding](encoding.html)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Serialize and deserialize unstructured JSON][ex-json-value] | [![serde-json-badge]][serde-json] | [![cat-encoding-badge]][cat-encoding] |
| [Deserialize a TOML configuration file][ex-toml-config] | [![toml-badge]][toml] | [![cat-encoding-badge]][cat-encoding] |
| [Percent-encode a string][ex-percent-encode] | [![url-badge]][url] | [![cat-encoding-badge]][cat-encoding] |
| [Encode a string as application/x-www-form-urlencoded][ex-urlencoded] | [![url-badge]][url] | [![cat-encoding-badge]][cat-encoding] |
| [Encode and decode hex][ex-hex-encode-decode] | [![data-encoding-badge]][data-encoding] | [![cat-encoding-badge]][cat-encoding] |
| [Encode and decode base64][ex-base64] | [![base64-badge]][base64] | [![cat-encoding-badge]][cat-encoding] |
| [Read CSV records][ex-csv-read] | [![csv-badge]][csv] | [![cat-encoding-badge]][cat-encoding] |
| [Read CSV records with different delimiter][ex-csv-delimiter] | [![csv-badge]][csv] | [![cat-encoding-badge]][cat-encoding] |
| [Filter CSV records matching a predicate][ex-csv-filter] | [![csv-badge]][csv] | [![cat-encoding-badge]][cat-encoding] |
| [Handle invalid CSV data with Serde][ex-invalid-csv] | [![csv-badge]][csv] [![serde-badge]][serde] | [![cat-encoding-badge]][cat-encoding] |
| [Serialize records to CSV][ex-serialize-csv] | [![csv-badge]][csv] | [![cat-encoding-badge]][cat-encoding] |
| [Serialize records to CSV using Serde][ex-csv-serde] | [![csv-badge]][csv] [![serde-badge]][serde] | [![cat-encoding-badge]][cat-encoding] |

## [Concurrency](concurrency.html)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Mutate the elements of an array in parallel][ex-rayon-iter-mut] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [Test in parallel if any or all elements of a collection match a given predicate][ex-rayon-any-all] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [Search items using given predicate in parallel][ex-rayon-parallel-search] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [Sort a vector in parallel][ex-rayon-parallel-sort] | [![rayon-badge]][rayon] [![rand-badge]][rand] | [![cat-concurrency-badge]][cat-concurrency] |
| [Map-reduce in parallel][ex-rayon-map-reduce] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [Generate jpg thumbnails in parallel][ex-rayon-thumbnails] | [![rayon-badge]][rayon] [![glob-badge]][glob] [![image-badge]][image] | [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem] |
| [Spawn a short-lived thread][ex-crossbeam-spawn] | [![crossbeam-badge]][crossbeam] | [![cat-concurrency-badge]][cat-concurrency] |
| [Draw fractal dispatching work to a thread pool][ex-threadpool-fractal] | [![threadpool-badge]][threadpool] [![num-badge]][num] [![num_cpus-badge]][num_cpus] [![image-badge]][image] | [![cat-concurrency-badge]][cat-concurrency][![cat-science-badge]][cat-science][![cat-rendering-badge]][cat-rendering] |
| [Calculate SHA1 sum of *.iso files concurrently][ex-threadpool-walk]  | [![threadpool-badge]][threadpool] [![walkdir-badge]][walkdir] [![num_cpus-badge]][num_cpus] [![ring-badge]][ring] | [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem] |

## [Networking](net.html)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Parse a URL from a string to a `Url` type][ex-url-parse] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Create a base URL by removing path segments][ex-url-base] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Create new URLs from a base URL][ex-url-new-from-base] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Extract the URL origin (scheme / host / port)][ex-url-origin] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Remove fragment identifiers and query pairs from a URL][ex-url-rm-frag] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Make a HTTP GET request][ex-url-basic] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |
| [Download a file to a temporary directory][ex-url-download] | [![reqwest-badge]][reqwest] [![tempdir-badge]][tempdir] | [![cat-net-badge]][cat-net] [![cat-filesystem-badge]][cat-filesystem] |
| [Query the GitHub API][ex-rest-get] | [![reqwest-badge]][reqwest] [![serde-badge]][serde] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding] |
| [Consume a paginated RESTful API][ex-paginated-api] | [![reqwest-badge]][reqwest] [![serde-badge]][serde] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding] |
| [Check if an API resource exists][ex-rest-head] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |
| [Set custom headers and URL parameters for a REST request][ex-rest-custom-params] | [![reqwest-badge]][reqwest] [![hyper-badge]][hyper] [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Create and delete Gist with GitHub API][ex-rest-post] | [![reqwest-badge]][reqwest] [![serde-badge]][serde] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding] |
| [POST a file to paste-rs][ex-file-post] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |
| [Listen on unused port TCP/IP][ex-random-port-tcp] | [![std-badge]][std] | [![cat-net-badge]][cat-net] |
| [Extract all links from a webpage HTML][ex-extract-links-webpage] | [![reqwest-badge]][reqwest] [![select-badge]][select] | [![cat-net-badge]][cat-net] |
| [Check webpage for broken links][ex-check-broken-links] | [![reqwest-badge]][reqwest] [![select-badge]][select] [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Extract all unique links from a MediaWiki markup][ex-extract-mediawiki-links] | [![reqwest-badge]][reqwest] [![regex-badge]][regex] | [![cat-net-badge]][cat-net] |
| [Make a partial download with HTTP range headers][ex-progress-with-range] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |

## [Application development](app.html)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Parse command line arguments][ex-clap-basic] | [![clap-badge]][clap] | [![cat-command-line-badge]][cat-command-line] |
| [Decompress a tarball][ex-tar-decompress] | [![flate2-badge]][flate2] [![tar-badge]][tar] | [![cat-compression-badge]][cat-compression] |
| [Compress a directory into a tarball][ex-tar-compress] | [![flate2-badge]][flate2] [![tar-badge]][tar] | [![cat-compression-badge]][cat-compression] |
| [Decompress a tarball while removing a prefix from the paths][ex-tar-strip-prefix] | [![flate2-badge]][flate2] [![tar-badge]][tar] | [![cat-compression-badge]][cat-compression] |
| [Avoid writing and reading from a same file][ex-avoid-read-write] | [![same_file-badge]][same_file] | [![cat-filesystem-badge]][cat-filesystem] |
| [Find loops for a given path][ex-find-file-loops] | [![same_file-badge]][same_file] | [![cat-filesystem-badge]][cat-filesystem] |
| [Recursively find duplicate file names][ex-dedup-filenames] | [![walkdir-badge]][walkdir] | [![cat-filesystem-badge]][cat-filesystem] |
| [Recursively find all files with given predicate][ex-file-predicate] | [![walkdir-badge]][walkdir] | [![cat-filesystem-badge]][cat-filesystem] |
| [Traverse directories while skipping dotfiles][ex-file-skip-dot] | [![walkdir-badge]][walkdir] | [![cat-filesystem-badge]][cat-filesystem] |
| [Recursively calculate file sizes at given depth][ex-file-sizes] | [![walkdir-badge]][walkdir] | [![cat-filesystem-badge]][cat-filesystem] |
| [Find all png files recursively][ex-glob-recursive] | [![glob-badge]][glob] | [![cat-filesystem-badge]][cat-filesystem] |
| [Find all files with given pattern ignoring filename case][ex-glob-with] | [![glob-badge]][glob] | [![cat-filesystem-badge]][cat-filesystem] |
| [Parse and increment a version string][ex-semver-increment] | [![semver-badge]][semver] | [![cat-config-badge]][cat-config] |
| [Parse a complex version string][ex-semver-complex] | [![semver-badge]][semver] | [![cat-config-badge]][cat-config] |
| [Check if given version is pre-release][ex-semver-prerelease] | [![semver-badge]][semver] | [![cat-config-badge]][cat-config] |
| [Check external command version for compatibility][ex-semver-command] | [![semver-badge]][semver] | [![cat-text-processing-badge]][cat-text-processing] [![cat-os-badge]][cat-os]

## [Logging](logging.html)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Log a debug message to the console][ex-log-debug] | [![log-badge]][log] [![env_logger-badge]][env_logger] | [![cat-debugging-badge]][cat-debugging] |
| [Log an error message to the console][ex-log-error] | [![log-badge]][log] [![env_logger-badge]][env_logger] | [![cat-debugging-badge]][cat-debugging] |
| [Enable log levels per module][ex-log-mod] | [![log-badge]][log] [![env_logger-badge]][env_logger] | [![cat-debugging-badge]][cat-debugging] |
| [Log to stdout instead of stderr][ex-log-stdout] | [![log-badge]][log] [![env_logger-badge]][env_logger] | [![cat-debugging-badge]][cat-debugging] |
| [Log messages with a custom logger][ex-log-custom-logger] | [![log-badge]][log] | [![cat-debugging-badge]][cat-debugging] |
| [Use a custom environment variable to set up logging][ex-log-env-variable] | [![log-badge]][log] [![env_logger-badge]][env_logger] | [![cat-debugging-badge]][cat-debugging] |
| [Include timestamp in log messages][ex-log-timestamp] | [![log-badge]][log] [![env_logger-badge]][env_logger] [![chrono-badge]][chrono] | [![cat-debugging-badge]][cat-debugging] |
| [Log to the Unix syslog][ex-log-syslog] | [![log-badge]][log] [![syslog-badge]][syslog] | [![cat-debugging-badge]][cat-debugging] |
| [Log messages to a custom location][ex-log-custom] | [![log-badge]][log] [![log4rs-badge]][log4rs] | [![cat-debugging-badge]][cat-debugging] |

## [Build Time Tooling](build_tools.html)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Compile and link statically to a bundled C library][ex-cc-static-bundled] | [![cc-badge]][cc] | [![cat-development-tools-badge]][cat-development-tools] |
| [Compile and link statically to a bundled C++ library][ex-cc-static-bundled-cpp]  | [![cc-badge]][cc] | [![cat-development-tools-badge]][cat-development-tools] |
| [Compile a C library while setting custom defines][ex-cc-custom-defines] | [![cc-badge]][cc] | [![cat-development-tools-badge]][cat-development-tools] |

{{#include links.md}}

<!-- Examples -->

[ex-avoid-read-write]: app.html#ex-avoid-read-write
[ex-base64]: encoding.html#ex-base64
[ex-bitflags]: basics.html#ex-bitflags
[ex-byteorder-le]: basics.html#ex-byteorder-le
[ex-cc-static-bundled]: build_tools.html#ex-cc-static-bundled
[ex-cc-static-bundled-cpp]: build_tools.html#ex-cc-static-bundled-cpp
[ex-cc-custom-defines]: build_tools.html#ex-cc-custom-defines
[ex-check-broken-links]: net.html#ex-check-broken-links
[ex-check-cpu-cores]: basics.html#ex-check-cpu-cores
[ex-clap-basic]: app.html#ex-clap-basic
[ex-crossbeam-spawn]: concurrency.html#ex-crossbeam-spawn
[ex-csv-serde]: encoding.html#ex-csv-serde
[ex-csv-read]: encoding.html#ex-csv-read
[ex-csv-delimiter]: encoding.html#ex-csv-delimiter
[ex-format-datetime]: basics.html#ex-format-datetime
[ex-threadpool-fractal]: concurrency.html#ex-threadpool-fractal
[ex-threadpool-walk]: concurrency.html#ex-threadpool-walk
[ex-dedup-filenames]: app.html#ex-dedup-filenames
[ex-error-chain-backtrace]: basics.html#ex-error-chain-backtrace
[ex-measure-elapsed-time]: basics.html#ex-measure-elapsed-time
[ex-extract-links-webpage]: net.html#ex-extract-links-webpage
[ex-extract-hashtags]: basics.html#ex-extract-hashtags
[ex-extract-mediawiki-links]: net.html#ex-extract-mediawiki-links
[ex-file-post]: net.html#ex-file-post
[ex-file-predicate]: app.html#ex-file-predicate
[ex-file-skip-dot]: app.html#ex-file-skip-dot
[ex-file-sizes]: app.html#ex-file-sizes
[ex-find-file-loops]: app.html#ex-find-file-loops
[ex-global-mut-state]: basics.html#ex-global-mut-state
[ex-glob-recursive]: app.html#ex-glob-recursive
[ex-glob-with]: app.html#ex-glob-with
[ex-hex-encode-decode]: encoding.html#ex-hex-encode-decode
[ex-json-value]: encoding.html#ex-json-value
[ex-lazy-constant]: basics.html#ex-lazy-constant
[ex-log-custom-logger]: logging.html#ex-log-custom-logger
[ex-log-custom]: logging.html#ex-log-custom
[ex-log-debug]: logging.html#ex-log-debug
[ex-log-env-variable]: logging.html#ex-log-env-variable
[ex-log-error]: logging.html#ex-log-error
[ex-log-mod]: logging.html#ex-log-mod
[ex-log-syslog]: logging.html#ex-log-syslog
[ex-log-timestamp]: logging.html#ex-log-timestamp
[ex-log-stdout]: logging.html#ex-log-stdout
[ex-invalid-csv]: encoding.html#ex-invalid-csv
[ex-paginated-api]: net.html#ex-paginated-api
[ex-parse-subprocess-output]: basics.html#ex-parse-subprocess-output
[ex-parse-subprocess-input]: basics.html#ex-parse-subprocess-input
[ex-pbkdf2]: basics.html#ex-pbkdf2
[ex-run-piped-external-commands]: basics.html#ex-run-piped-external-commands
[ex-redirect-stdout-stderr-same-file]: basics.html#ex-redirect-stdout-stderr-same-file
[ex-verify-extract-email]: basics.html#ex-verify-extract-email
[ex-percent-encode]: encoding.html#ex-percent-encode
[ex-phone]: basics.html#ex-phone
[ex-rand-custom]: basics.html#ex-rand-custom
[ex-rand-dist]: basics.html#ex-rand-dist
[ex-rand]: basics.html#ex-rand
[ex-rand-range]: basics.html#ex-rand-range
[ex-random-port-tcp]: net.html#ex-random-port-tcp
[ex-rayon-iter-mut]: concurrency.html#ex-rayon-iter-mut
[ex-rayon-any-all]: concurrency.html#ex-rayon-any-all
[ex-rayon-map-reduce]: concurrency.html#ex-rayon-map-reduce
[ex-rayon-parallel-search]: concurrency.html#ex-rayon-parallel-search
[ex-rayon-parallel-sort]: concurrency.html#ex-rayon-parallel-sort
[ex-rayon-thumbnails]: concurrency.html#ex-rayon-thumbnails
[ex-regex-filter-log]: basics.html#ex-regex-filter-log
[ex-rest-custom-params]: net.html#ex-rest-custom-params
[ex-rest-get]: net.html#ex-rest-get
[ex-rest-head]: net.html#ex-rest-head
[ex-rest-post]: net.html#ex-rest-post
[ex-semver-command]: app.html#ex-semver-command
[ex-semver-complex]: app.html#ex-semver-complex
[ex-semver-increment]: app.html#ex-semver-increment
[ex-semver-prerelease]: app.html#ex-semver-prerelease
[ex-serialize-csv]: encoding.html#ex-serialize-csv
[ex-sha-digest]: basics.html#ex-sha-digest
[ex-hmac]: basics.html#ex-hmac
[ex-std-read-lines]: basics.html#ex-std-read-lines
[ex-tar-compress]: app.html#ex-tar-compress
[ex-tar-decompress]: app.html#ex-tar-decompress
[ex-tar-strip-prefix]: app.html#ex-tar-strip-prefix
[ex-toml-config]: encoding.html#ex-toml-config
[ex-url-base]: net.html#ex-url-base
[ex-url-basic]: net.html#ex-url-basic
[ex-url-download]: net.html#ex-url-download
[ex-url-new-from-base]: net.html#ex-url-new-from-base
[ex-url-origin]: net.html#ex-url-origin
[ex-url-parse]: net.html#ex-url-parse
[ex-url-rm-frag]: net.html#ex-url-rm-frag
[ex-urlencoded]: encoding.html#ex-urlencoded
[ex-progress-with-range]: net.html#ex-progress-with-range
[ex-random-file-access]: basics.html#ex-random-file-access
[ex-regex-replace-named]: basics.html#ex-regex-replace-named
[ex-csv-filter]: encoding.html#ex-csv-filter
