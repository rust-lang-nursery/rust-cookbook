# Cookin' with Rust

A practical guide to the Rust crate ecosystem.

This _Rust Cookbook_ (a.k.a. _Cookin' with Rust_), is a collection of
simple examples that demonstrate good practices to accomplish common
programming tasks, using the crates of the Rust ecosystem.

[Read more about _Cookin' with Rust_](about.html), including tips for
how to read the book, how to use the examples, notes on conventions.

## Contributing

This project is intented to be easy for new Rust programmers to
contribute to, and an easy to way get involved with the Rust
community. It needs and welcomes help.

For details see [CONTRIBUTING.md] on GitHub.

[CONTRIBUTING.md]: https://github.com/brson/rust-cookbook/blob/master/CONTRIBUTING.md

## Basics

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Read lines of strings from a file][ex-std-read-lines] | [![std-badge]][std] | [![cat-filesystem-badge]][cat-filesystem] |
| [Read and write integers in little-endian byte order][ex-byteorder-le] | [![byteorder-badge]][byteorder] | [![cat-encoding-badge]][cat-encoding] |
| [Generate random floating point numbers][ex-rand-float] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random values on a custom type][ex-rand-custom] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Construct a graph of objects][ex-petgraph-basic] | [![petgraph-badge]][petgraph] | [![cat-science-badge]][cat-science] |

## Encoding

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Serialize and deserialize unstructured JSON][ex-json-basic] | [![serde-json-badge]][serde-json] | [![cat-encoding-badge]][cat-encoding] |
| [Deserialize an unstructured TOML configuration file][ex-toml-basic] | [![toml-badge]][toml] | [![cat-encoding-badge]][cat-encoding] |

## Concurrency

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Mutate the elements of an array in parallel][ex-rayon-iter-mut] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |

## Application development

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

[cat-command-line-badge]: https://img.shields.io/badge/-command_line-orange.svg
[cat-command-line]: https://crates.io/categories/command-line-interface
[cat-concurrency-badge]: https://img.shields.io/badge/-concurrency-orange.svg
[cat-concurrency]: https://crates.io/categories/concurrency
[cat-filesystem-badge]: https://img.shields.io/badge/-file_io-orange.svg
[cat-filesystem]: https://crates.io/categories/filesystem
[cat-science-badge]: https://img.shields.io/badge/-rand-orange.svg
[cat-science]: https://crates.io/categories/science
[cat-encoding-badge]: https://img.shields.io/badge/-encoding-orange.svg
[cat-encoding]: https://crates.io/categories/encoding

<!-- Crates -->

[byteorder-badge]: https://img.shields.io/crates/v/byteorder.svg?label=byteorder
[byteorder]: https://docs.rs/byteorder/1.0.0/byteorder/
[clap-badge]: https://img.shields.io/crates/v/clap.svg?label=clap
[clap]: https://docs.rs/clap/2.22.0/rand/
[serde-json-badge]: https://img.shields.io/crates/v/serde_json.svg?label=serde_json
[serde-json]: https://docs.serde.rs/serde_json/
[petgraph-badge]: https://img.shields.io/crates/v/petgraph.svg?label=petgraph
[petgraph]: https://docs.rs/petgraph/0.4.3/petgraph/
[rand-badge]: https://img.shields.io/crates/v/rand.svg?label=rand
[rand]: https://docs.rs/rand/0.3.15/rand/
[rayon-badge]: https://img.shields.io/crates/v/rayon.svg?label=rayon
[rayon]: https://docs.rs/rayon/0.6.0/rayon/
[std-badge]: https://img.shields.io/badge/std-1.17.0-blue.svg
[std]: https://doc.rust-lang.org/std
[toml-badge]: https://img.shields.io/crates/v/toml.svg?label=toml
[toml]: https://docs.rs/toml/0.3.0/toml/

<!-- Examples -->

[ex-byteorder-le]: basics.html#ex-byteorder-le
[ex-clap-basic]: todo
[ex-json-basic]: todo
[ex-petgraph-basic]: basics.html#ex-petgraph-basic
[ex-rand-custom]: basics.html#ex-rand-custom
[ex-rand-float]: basics.html#ex-rand-float
[ex-rayon-iter-mut]: todo
[ex-std-read-lines]: basics.html#ex-std-read-lines
[ex-toml-basic]: todo
