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

[CONTRIBUTING.md]: https://github.com/brson/rust-cookbook/blob/master/CONTRIBUTING.md

## [Basics](basics.html)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Read lines of strings from a file][ex-std-read-lines] | [![std-badge]][std] | [![cat-filesystem-badge]][cat-filesystem] |
| [Read and write integers in little-endian byte order][ex-byteorder-le] | [![byteorder-badge]][byteorder] | [![cat-encoding-badge]][cat-encoding] |
| [Generate random floating point numbers][ex-rand-float] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random numbers within a range][ex-rand-range] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random numbers with normal distribution][ex-rand-dist] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random values of a custom type][ex-rand-custom] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Run an External Command and Process Stdout][ex-parse-subprocess-output] | [![regex-badge]][regex] | [![cat-os-badge]][cat-os] [![cat-text-processing-badge]][cat-text-processing] |

## [Encoding](encoding.html)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Serialize and deserialize unstructured JSON][ex-json-value] | [![serde-json-badge]][serde-json] | [![cat-encoding-badge]][cat-encoding] |
| [Deserialize a TOML configuration file][ex-toml-config] | [![toml-badge]][toml] | [![cat-encoding-badge]][cat-encoding] |
| [Percent-encode a string][ex-percent-encode] | [![url-badge]][url] | [![cat-encoding-badge]][cat-encoding] |
| [Encode and decode base64][ex-base64] | [![base64-badge]][base64] | [![cat-encoding-badge]][cat-encoding] |

## [Concurrency](concurrency.html)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Mutate the elements of an array in parallel][ex-rayon-iter-mut] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |

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
| [Check if an API Resource Exists][ex-rest-head] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |
| [Create and delete Gist with GitHub API][ex-rest-post] | [![reqwest-badge]][reqwest] [![serde-badge]][serde] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding] |

## [Application development](app.html)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Parse command line arguments][ex-clap-basic] | [![clap-badge]][clap] | [![cat-command-line-badge]][cat-command-line] |
| [Log a debug message to the console][ex-log-debug] | [![log-badge]][log] [![env_logger-badge]][env_logger] | [![cat-debugging-badge]][cat-debugging] |
| [Log an error message to the console][ex-log-error] | [![log-badge]][log] [![env_logger-badge]][env_logger] | [![cat-debugging-badge]][cat-debugging] |
| [Enable log levels per module][ex-log-mod] | [![log-badge]][log] [![env_logger-badge]][env_logger] | [![cat-debugging-badge]][cat-debugging] |
| [Log messages with a custom logger][ex-log-custom-logger] | [![log-badge]][log] | [![cat-debugging-badge]][cat-debugging] |
| [Log to the Unix syslog][ex-log-syslog] | [![log-badge]][log] [![syslog-badge]][syslog] | [![cat-debugging-badge]][cat-debugging] |
| [Log messages to a custom location][ex-log-custom] | [![log-badge]][log] | [![cat-debugging-badge]][cat-debugging] |


<!--

Links, in a few categories. Follow the existing structure.

Individual pages contain a subset of these exact links, depending on
the crates and categories of their examples.

Keep lines sorted.

-->

<!-- Categories -->

[cat-command-line-badge]: https://img.shields.io/badge/-command_line-red.svg
[cat-command-line]: https://crates.io/categories/command-line-interface
[cat-concurrency-badge]: https://img.shields.io/badge/-concurrency-red.svg
[cat-concurrency]: https://crates.io/categories/concurrency
[cat-debugging-badge]: https://img.shields.io/badge/-debugging-red.svg
[cat-debugging]: https://crates.io/categories/debugging
[cat-encoding-badge]: https://img.shields.io/badge/-encoding-red.svg
[cat-encoding]: https://crates.io/categories/encoding
[cat-filesystem-badge]: https://img.shields.io/badge/-filesystem-red.svg
[cat-filesystem]: https://crates.io/categories/filesystem
[cat-net-badge]: https://img.shields.io/badge/-net-red.svg
[cat-net]: https://crates.io/categories/network-programming
[cat-science-badge]: https://img.shields.io/badge/-science-red.svg
[cat-science]: https://crates.io/categories/science
[cat-os-badge]: https://img.shields.io/badge/-os-red.svg
[cat-os]: https://crates.io/categories/os
[cat-text-processing-badge]: https://img.shields.io/badge/-text_processing-red.svg
[cat-text-processing]: https://crates.io/categories/text-processing

<!-- Crates -->

[byteorder-badge]: https://img.shields.io/crates/v/byteorder.svg?label=byteorder
[byteorder]: https://docs.rs/byteorder/
[clap-badge]: https://img.shields.io/crates/v/clap.svg?label=clap
[clap]: https://docs.rs/clap/
[env_logger-badge]: https://img.shields.io/crates/v/env_logger.svg?label=env_logger
[env_logger]: https://docs.rs/env_logger/
[log-badge]: https://img.shields.io/crates/v/log.svg?label=log
[log]: https://docs.rs/log/
[rand-badge]: https://img.shields.io/crates/v/rand.svg?label=rand
[rand]: https://docs.rs/rand/
[rayon-badge]: https://img.shields.io/crates/v/rayon.svg?label=rayon
[rayon]: https://docs.rs/rayon/
[reqwest-badge]: https://img.shields.io/crates/v/reqwest.svg?label=reqwest
[reqwest]: https://docs.rs/reqwest/
[serde-badge]: https://img.shields.io/crates/v/serde.svg?label=serde
[serde-json-badge]: https://img.shields.io/crates/v/serde_json.svg?label=serde_json
[serde-json]: https://docs.serde.rs/serde_json/
[serde]: https://docs.rs/serde/
[std-badge]: https://img.shields.io/badge/std-1.17.0-blue.svg
[std]: https://doc.rust-lang.org/std
[syslog-badge]: https://img.shields.io/crates/v/syslog.svg?label=syslog
[syslog]: https://docs.rs/syslog/
[tempdir-badge]: https://img.shields.io/crates/v/tempdir.svg?label=tempdir
[tempdir]: https://docs.rs/tempdir/
[toml-badge]: https://img.shields.io/crates/v/toml.svg?label=toml
[toml]: https://docs.rs/toml/
[url-badge]: https://img.shields.io/crates/v/url.svg?label=url
[url]: https://docs.rs/url/
[base64-badge]: https://img.shields.io/crates/v/base64.svg?label=base64
[base64]: https://docs.rs/base64/
[regex]: https://docs.rs/regex/
[regex-badge]: https://img.shields.io/crates/v/regex.svg?label=regex

<!-- Examples -->

[ex-byteorder-le]: basics.html#ex-byteorder-le
[ex-clap-basic]: app.html#ex-clap-basic
[ex-json-value]: encoding.html#ex-json-value
[ex-log-custom-logger]: app.html#ex-log-custom-logger
[ex-log-custom]: app.html#ex-log-custom
[ex-log-debug]: app.html#ex-log-debug
[ex-log-error]: app.html#ex-log-error
[ex-log-mod]: app.html#ex-log-mod
[ex-log-syslog]: app.html#ex-log-syslog
[ex-percent-encode]: encoding.html#ex-percent-encode
[ex-rand-custom]: basics.html#ex-rand-custom
[ex-rand-dist]: basics.html#ex-rand-dist
[ex-rand-float]: basics.html#ex-rand-float
[ex-rand-range]: basics.html#ex-rand-range
[ex-rayon-iter-mut]: concurrency.html#ex-rayon-iter-mut
[ex-rest-head]: net.html#ex-rest-head
[ex-rest-get]: net.html#ex-rest-get
[ex-rest-post]: net.html#ex-rest-post
[ex-std-read-lines]: basics.html#ex-std-read-lines
[ex-toml-config]: encoding.html#ex-toml-config
[ex-url-base]: net.html#ex-url-base
[ex-url-basic]: net.html#ex-url-basic
[ex-url-download]: net.html#ex-url-download
[ex-url-new-from-base]: net.html#ex-url-new-from-base
[ex-url-origin]: net.html#ex-url-origin
[ex-url-parse]: net.html#ex-url-parse
[ex-url-rm-frag]: net.html#ex-url-rm-frag
[ex-base64]: encoding.html#ex-base64
[ex-parse-subprocess-output]: basics.html#ex-parse-subprocess-output
