# Cookin' with Rust

This _Rust Cookbook_ is a collection of
simple examples that demonstrate good practices to accomplish common
programming tasks, using the crates of the Rust ecosystem.

[Read more about _Rust Cookbook_](about.html), including tips for
how to read the book, how to use the examples, and notes on conventions.

## Contributing

This project is intented to be easy for new Rust programmers to
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
| [Generate random values on a custom type][ex-rand-custom] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |

## [Encoding](encoding.html)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Serialize and deserialize unstructured JSON][ex-json-value] | [![serde-json-badge]][serde-json] | [![cat-encoding-badge]][cat-encoding] |
| [Deserialize a TOML configuration file][ex-toml-config] | [![toml-badge]][toml] | [![cat-encoding-badge]][cat-encoding] |

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
| [Serialize a `Url`][ex-url-serialize] | [![url-badge]][url] [![serde-badge]][serde] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]|
| [Make a HTTP GET request after parsing a URL][ex-url-reqwest] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |

## [Application development](app.html)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Parse command line arguments][ex-clap-basic] | [![clap-badge]][clap] | [![cat-command-line-badge]][cat-command-line] |

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
[cat-filesystem-badge]: https://img.shields.io/badge/-filesystem-red.svg
[cat-filesystem]: https://crates.io/categories/filesystem
[cat-science-badge]: https://img.shields.io/badge/-science-red.svg
[cat-science]: https://crates.io/categories/science
[cat-encoding-badge]: https://img.shields.io/badge/-encoding-red.svg
[cat-encoding]: https://crates.io/categories/encoding
[cat-net-badge]: https://img.shields.io/badge/-net-red.svg
[cat-net]: https://crates.io/categories/network-programming

<!-- Crates -->

[byteorder-badge]: https://img.shields.io/crates/v/byteorder.svg?label=byteorder
[byteorder]: https://docs.rs/byteorder/
[clap-badge]: https://img.shields.io/crates/v/clap.svg?label=clap
[clap]: https://docs.rs/clap/
[serde-json-badge]: https://img.shields.io/crates/v/serde_json.svg?label=serde_json
[serde-json]: https://docs.serde.rs/serde_json/
[rand-badge]: https://img.shields.io/crates/v/rand.svg?label=rand
[rand]: https://docs.rs/rand/
[rayon-badge]: https://img.shields.io/crates/v/rayon.svg?label=rayon
[rayon]: https://docs.rs/rayon/
[reqwest-badge]: https://img.shields.io/crates/v/reqwest.svg?label=reqwest
[reqwest]: https://docs.rs/url/
[serde-badge]: https://img.shields.io/crates/v/serde.svg?label=serde
[serde]: https://docs.rs/serde/
[std-badge]: https://img.shields.io/badge/std-1.17.0-blue.svg
[std]: https://doc.rust-lang.org/std
[toml-badge]: https://img.shields.io/crates/v/toml.svg?label=toml
[toml]: https://docs.rs/toml/
[url-badge]: https://img.shields.io/crates/v/url.svg?label=url
[url]: https://docs.rs/url/

<!-- Examples -->

[ex-byteorder-le]: basics.html#ex-byteorder-le
[ex-clap-basic]: app.html#ex-clap-basic
[ex-json-value]: encoding.html#ex-json-value
[ex-rand-custom]: basics.html#ex-rand-custom
[ex-rand-float]: basics.html#ex-rand-float
[ex-rayon-iter-mut]: concurrency.html#ex-rayon-iter-mut
[ex-std-read-lines]: basics.html#ex-std-read-lines
[ex-toml-config]: encoding.html#ex-toml-config
[ex-url-parse]: net.html#ex-url-parse
[ex-url-base]: net.html#ex-url-base
[ex-url-new-from-base]: net.html#ex-url-new-from-base
[ex-url-origin]: net.html#ex-url-origin
[ex-url-rm-frag]: net.html#ex-url-rm-frag
[ex-url-serialize]: net.html#ex-url-serialize
[ex-url-reqwest]: net.html#ex-url-reqwest
