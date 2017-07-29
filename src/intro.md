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
| [Generate random number distributions][ex-rand-dist] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random values of a custom type][ex-rand-custom] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Run an external command and process stdout][ex-parse-subprocess-output] | [![regex-badge]][regex] | [![cat-os-badge]][cat-os] [![cat-text-processing-badge]][cat-text-processing] |
| [Filter a log file by matching multiple regular expressions][ex-regex-filter-log] | [![regex-badge]][regex] | [![cat-text-processing-badge]][cat-text-processing]
| [Declare lazily evaluated constant][ex-lazy-constant] | [![lazy_static-badge]][lazy_static] | [![cat-caching-badge]][cat-caching] [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Maintain global mutable state][ex-global-mut-state] | [![lazy_static-badge]][lazy_static] | [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Access a file randomly using a memory map][ex-random-file-access] | [![memmap-badge]][memmap] | [![cat-filesystem-badge]][cat-filesystem] |
| [Define and operate on a type represented as a bitfield][ex-bitflags] | [![bitflags-badge]][bitflags] | [![cat-no-std-badge]][cat-no-std] |
| [Extract a list of unique #Hashtags from a text][ex-extract-hashtags] | [![regex-badge]][regex] [![lazy_static-badge]][lazy_static] | [![cat-text-processing-badge]][cat-text-processing] |
| [Replace all occurrences of one text pattern with another pattern.][ex-regex-replace-named] | [![regex-badge]][regex] [![lazy_static-badge]][lazy_static] | [![cat-text-processing-badge]][cat-text-processing] |
| [Extract phone numbers from text][ex-phone] | [![regex-badge]][regex] | [![cat-text-processing-badge]][cat-text-processing] |
| [Calculate the SHA-256 digest of a file][ex-sha-digest] | [![ring-badge]][ring] [![data-encoding-badge]][data-encoding] | [![cat-cryptography-badge]][cat-cryptography] |


## [Encoding](encoding.html)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Serialize and deserialize unstructured JSON][ex-json-value] | [![serde-json-badge]][serde-json] | [![cat-encoding-badge]][cat-encoding] |
| [Deserialize a TOML configuration file][ex-toml-config] | [![toml-badge]][toml] | [![cat-encoding-badge]][cat-encoding] |
| [Percent-encode a string][ex-percent-encode] | [![url-badge]][url] | [![cat-encoding-badge]][cat-encoding] |
| [Encode a string as application/x-www-form-urlencoded][ex-urlencoded] | [![url-badge]][url] | [![cat-encoding-badge]][cat-encoding] |
| [Encode and decode hex][ex-hex-encode-decode] | [![data-encoding-badge]][data-encoding] | [![cat-encoding-badge]][cat-encoding] |
| [Encode and decode base64][ex-base64] | [![base64-badge]][base64] | [![cat-encoding-badge]][cat-encoding] |
| [Serialize records to CSV][ex-serialize-csv] | [![csv-badge]][csv] | [![cat-encoding-badge]][cat-encoding] |
| [Serialize records to CSV using Serde][ex-csv-serde] | [![csv-badge]][csv] [![serde-badge]][serde] | [![cat-encoding-badge]][cat-encoding] |
| [Handle invalid CSV data with Serde][ex-invalid-csv] | [![csv-badge]][csv] [![serde-badge]][serde] | [![cat-encoding-badge]][cat-encoding] |
| [Read CSV records with different delimeter][ex-csv-delimiter] | [![csv-badge]][csv] | [![cat-encoding-badge]][cat-encoding] |

## [Concurrency](concurrency.html)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Mutate the elements of an array in parallel][ex-rayon-iter-mut] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [Spawn a short-lived thread][ex-crossbeam-spawn] | [![crossbeam-badge]][crossbeam] | [![cat-concurrency-badge]][cat-concurrency] |
| [Draw fractal dispatching work to a thread pool][ex-threadpool-fractal] | [![threadpool-badge]][threadpool] [![num-badge]][num] [![num_cpus-badge]][num_cpus] [![image-badge]][image] | [![cat-concurrency-badge]][cat-concurrency][![cat-science-badge]][cat-science][![cat-rendering-badge]][cat-rendering] |

## [Networking](net.html)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Parse a URL from a string to a `Url` type][ex-url-parse] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Create a base URL by removing path segments][ex-url-base] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Create new URLs from a base URL][ex-url-new-from-base] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Extract the URL origin (scheme / host / port)][ex-url-origin] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Remove fragment identifiers and query pairs from a URL][ex-url-rm-frag] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Make a HTTP GET request after parsing a URL][ex-url-basic] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |
| [Download a file to a temporary directory][ex-url-download] | [![reqwest-badge]][reqwest] [![tempdir-badge]][tempdir] | [![cat-net-badge]][cat-net] [![cat-filesystem-badge]][cat-filesystem] |
| [Query the GitHub API][ex-rest-get] | [![reqwest-badge]][reqwest] [![serde-badge]][serde] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding] |
| [Consume a paginated RESTful API][ex-paginated-api] | [![reqwest-badge]][reqwest] [![serde-badge]][serde] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding] |
| [Check if an API resource exists][ex-rest-head] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |
| [Set custom headers and URL parameters for a REST request][ex-rest-custom-params] | [![reqwest-badge]][reqwest] [![hyper-badge]][hyper] [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Create and delete Gist with GitHub API][ex-rest-post] | [![reqwest-badge]][reqwest] [![serde-badge]][serde] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding] |
| [POST a file to paste-rs][ex-file-post] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |
| [Listen on unused port TCP/IP][ex-random-port-tcp] | [![std-badge]][std] | [![cat-net-badge]][cat-net] |
| [Extract all links from a webpage][ex-extract-links-webpage] | [![reqwest-badge]][reqwest] [![select-badge]][select] | [![cat-net-badge]][cat-net] |
| [Extract all unique links from a MediaWiki markup][ex-extract-mediawiki-links] | [![reqwest-badge]][reqwest] [![regex-badge]][regex] | [![cat-net-badge]][cat-net] |

## [Application development](app.html)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Parse command line arguments][ex-clap-basic] | [![clap-badge]][clap] | [![cat-command-line-badge]][cat-command-line] |
| [Decompress a tarball][ex-tar-decompress] | [![flate2-badge]][flate2] [![tar-badge]][tar] | [![cat-compression-badge]][cat-compression] |
| [Compress a directory into a tarball][ex-tar-compress] | [![flate2-badge]][flate2] [![tar-badge]][tar] | [![cat-compression-badge]][cat-compression] |
| [Decompress a tarball while removing a prefix from the paths][ex-tar-strip-prefix] | [![flate2-badge]][flate2] [![tar-badge]][tar] | [![cat-compression-badge]][cat-compression] |
| [Find loops for a given path][ex-find-file-loops] | [![same_file-badge]][same_file] | [![cat-filesystem-badge]][cat-filesystem] |
| [Recursively find duplicate file names][ex-dedup-filenames] | [![walkdir-badge]][walkdir] | [![cat-filesystem-badge]][cat-filesystem] |
| [Recursively find all files with given predicate][ex-file-predicate] | [![walkdir-badge]][walkdir] | [![cat-filesystem-badge]][cat-filesystem] |
| [Traverse directories while skipping dotfiles][ex-file-skip-dot] | [![walkdir-badge]][walkdir] | [![cat-filesystem-badge]][cat-filesystem] |
| [Recursively calculate file sizes at given depth][ex-file-sizes] | [![walkdir-badge]][walkdir] | [![cat-filesystem-badge]][cat-filesystem] |
| [Find all png files recursively][ex-glob-recursive] | [![glob-badge]][glob] | [![cat-filesystem-badge]][cat-filesystem] |
| [Find all files with given pattern ignoring filename case][ex-glob-with] | [![glob-badge]][glob] | [![cat-filesystem-badge]][cat-filesystem] |

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
| [Log messages to a custom location][ex-log-custom] | [![log-badge]][log] | [![cat-debugging-badge]][cat-debugging] |

<!--

Links, in a few categories. Follow the existing structure.

Individual pages contain a subset of these exact links, depending on
the crates and categories of their examples.

Keep lines sorted.

-->

<!-- Categories -->

[cat-no-std-badge]: https://badge-cache.kominick.com/badge/no_std--x.svg?style=social
[cat-no-std]: https://crates.io/categories/no-std
[cat-caching-badge]: https://badge-cache.kominick.com/badge/caching--x.svg?style=social
[cat-caching]: https://crates.io/categories/caching
[cat-command-line-badge]: https://badge-cache.kominick.com/badge/command_line--x.svg?style=social
[cat-command-line]: https://crates.io/categories/command-line-interface
[cat-compression-badge]: https://badge-cache.kominick.com/badge/compression--x.svg?style=social
[cat-compression]: https://crates.io/categories/compression
[cat-concurrency-badge]: https://badge-cache.kominick.com/badge/concurrency--x.svg?style=social
[cat-concurrency]: https://crates.io/categories/concurrency
[cat-cryptography-badge]: https://badge-cache.kominick.com/badge/cryptography--x.svg?style=social
[cat-cryptography]: https://crates.io/categories/cryptography
[cat-science-badge]: https://badge-cache.kominick.com/badge/science--x.svg?style=social
[cat-science]: https://crates.io/categories/science
[cat-rendering-badge]: https://badge-cache.kominick.com/badge/rendering--x.svg?style=social
[cat-rendering]: https://crates.io/categories/rendering
[cat-debugging-badge]: https://badge-cache.kominick.com/badge/debugging--x.svg?style=social
[cat-debugging]: https://crates.io/categories/debugging
[cat-encoding-badge]: https://badge-cache.kominick.com/badge/encoding--x.svg?style=social
[cat-encoding]: https://crates.io/categories/encoding
[cat-filesystem-badge]: https://badge-cache.kominick.com/badge/filesystem--x.svg?style=social
[cat-filesystem]: https://crates.io/categories/filesystem
[cat-net-badge]: https://badge-cache.kominick.com/badge/net--x.svg?style=social
[cat-net]: https://crates.io/categories/network-programming
[cat-os-badge]: https://badge-cache.kominick.com/badge/OS--x.svg?style=social
[cat-os]: https://crates.io/categories/os
[cat-rust-patterns-badge]: https://badge-cache.kominick.com/badge/rust_patterns--x.svg?style=social
[cat-rust-patterns]: https://crates.io/categories/rust-patterns
[cat-science-badge]: https://badge-cache.kominick.com/badge/science--x.svg?style=social
[cat-science]: https://crates.io/categories/science
[cat-text-processing-badge]: https://badge-cache.kominick.com/badge/text_processing--x.svg?style=social
[cat-text-processing]: https://crates.io/categories/text-processing

<!-- Crates -->

[base64-badge]: https://badge-cache.kominick.com/crates/v/base64.svg?label=base64
[base64]: https://docs.rs/base64/
[bitflags-badge]: https://badge-cache.kominick.com/crates/v/bitflags.svg?label=bitflags
[bitflags]: https://docs.rs/bitflags/
[byteorder-badge]: https://badge-cache.kominick.com/crates/v/byteorder.svg?label=byteorder
[byteorder]: https://docs.rs/byteorder/
[chrono-badge]: https://badge-cache.kominick.com/crates/v/chrono.svg?label=chrono
[chrono]: https://docs.rs/chrono/
[clap-badge]: https://badge-cache.kominick.com/crates/v/clap.svg?label=clap
[clap]: https://docs.rs/clap/
[crossbeam-badge]: https://badge-cache.kominick.com/crates/v/crossbeam.svg?label=crossbeam
[crossbeam]: https://docs.rs/crossbeam/
[csv-badge]: https://badge-cache.kominick.com/crates/v/csv.svg?label=csv
[csv]: https://docs.rs/csv/
[data-encoding-badge]: https://badge-cache.kominick.com/crates/v/data-encoding.svg?label=data-encoding
[data-encoding]: https://docs.rs/data-encoding/
[env_logger-badge]: https://badge-cache.kominick.com/crates/v/env_logger.svg?label=env_logger
[env_logger]: https://docs.rs/env_logger/
[flate2-badge]: https://badge-cache.kominick.com/crates/v/flate2.svg?label=flate2
[flate2]: https://docs.rs/flate2/
[glob-badge]:https://badge-cache.kominick.com/crates/v/glob.svg?label=glob
[glob]: https://docs.rs/glob/
[hyper-badge]: https://badge-cache.kominick.com/crates/v/hyper.svg?label=hyper
[hyper]: https://docs.rs/hyper/
[lazy_static-badge]: https://badge-cache.kominick.com/crates/v/lazy_static.svg?label=lazy_static
[lazy_static]: https://docs.rs/lazy_static/
[log-badge]: https://badge-cache.kominick.com/crates/v/log.svg?label=log
[log]: https://docs.rs/log/
[rand-badge]: https://badge-cache.kominick.com/crates/v/rand.svg?label=rand
[rand]: https://docs.rs/rand/
[rayon-badge]: https://badge-cache.kominick.com/crates/v/rayon.svg?label=rayon
[rayon]: https://docs.rs/rayon/
[image-badge]: https://badge-cache.kominick.com/crates/v/image.svg?label=image
[image]: https://docs.rs/image/
[num-badge]: https://badge-cache.kominick.com/crates/v/num.svg?label=num
[num]: https://docs.rs/num/
[num_cpus-badge]: https://badge-cache.kominick.com/crates/v/num_cpus.svg?label=num_cpus
[num_cpus]: https://docs.rs/num_cpus/
[threadpool-badge]: https://badge-cache.kominick.com/crates/v/threadpool.svg?label=threadpool
[threadpool]: https://docs.rs/threadpool/
[regex-badge]: https://badge-cache.kominick.com/crates/v/regex.svg?label=regex
[regex]: https://docs.rs/regex/
[reqwest-badge]: https://badge-cache.kominick.com/crates/v/reqwest.svg?label=reqwest
[reqwest]: https://docs.rs/reqwest/
[ring-badge]: https://badge-cache.kominick.com/crates/v/ring.svg?label=ring
[ring]: https://docs.rs/ring/
[same_file-badge]: https://badge-cache.kominick.com/crates/v/same_file.svg?label=same_file
[same_file]: https://docs.rs/same-file/
[select-badge]: https://badge-cache.kominick.com/crates/v/select.svg?label=select
[select]: https://docs.rs/select/
[serde-badge]: https://badge-cache.kominick.com/crates/v/serde.svg?label=serde
[serde-json-badge]: https://badge-cache.kominick.com/crates/v/serde_json.svg?label=serde_json
[serde-json]: https://docs.serde.rs/serde_json/
[serde]: https://docs.rs/serde/
[std-badge]: https://badge-cache.kominick.com/badge/std-1.17.0-blue.svg
[std]: https://doc.rust-lang.org/std
[syslog-badge]: https://badge-cache.kominick.com/crates/v/syslog.svg?label=syslog
[syslog]: https://docs.rs/syslog/
[tar-badge]: https://badge-cache.kominick.com/crates/v/tar.svg?label=tar
[tar]: https://docs.rs/tar/
[tempdir-badge]: https://badge-cache.kominick.com/crates/v/tempdir.svg?label=tempdir
[tempdir]: https://docs.rs/tempdir/
[toml-badge]: https://badge-cache.kominick.com/crates/v/toml.svg?label=toml
[toml]: https://docs.rs/toml/
[url-badge]: https://badge-cache.kominick.com/crates/v/url.svg?label=url
[url]: https://docs.rs/url/
[walkdir-badge]: https://badge-cache.kominick.com/crates/v/walkdir.svg?label=walkdir
[walkdir]: https://docs.rs/walkdir/
[memmap-badge]: https://badge-cache.kominick.com/crates/v/memmap.svg?label=memmap
[memmap]: https://docs.rs/memmap/

<!-- Examples -->

[ex-base64]: encoding.html#ex-base64
[ex-bitflags]: basics.html#ex-bitflags
[ex-byteorder-le]: basics.html#ex-byteorder-le
[ex-clap-basic]: app.html#ex-clap-basic
[ex-crossbeam-spawn]: concurrency.html#ex-crossbeam-spawn
[ex-csv-serde]: encoding.html#ex-csv-serde
[ex-csv-delimiter]: encoding.html#ex-csv-delimiter
[ex-threadpool-fractal]: concurrency.html#ex-threadpool-fractal
[ex-dedup-filenames]: app.html#ex-dedup-filenames
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
[ex-percent-encode]: encoding.html#ex-percent-encode
[ex-phone]: basics.html#ex-phone
[ex-rand-custom]: basics.html#ex-rand-custom
[ex-rand-dist]: basics.html#ex-rand-dist
[ex-rand]: basics.html#ex-rand
[ex-rand-range]: basics.html#ex-rand-range
[ex-random-port-tcp]: net.html#ex-random-port-tcp
[ex-rayon-iter-mut]: concurrency.html#ex-rayon-iter-mut
[ex-regex-filter-log]: basics.html#ex-regex-filter-log
[ex-rest-custom-params]: net.html#ex-rest-custom-params
[ex-rest-get]: net.html#ex-rest-get
[ex-rest-head]: net.html#ex-rest-head
[ex-rest-post]: net.html#ex-rest-post
[ex-serialize-csv]: encoding.html#ex-serialize-csv
[ex-sha-digest]: basics.html#ex-sha-digest
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
[ex-random-file-access]: basics.html#ex-random-file-access
[ex-regex-replace-named]: basics.html#ex-regex-replace-named
