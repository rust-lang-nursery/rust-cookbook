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
| [Read lines of strings from a file][ex-std-read-lines] | [![std-badge]][std] | [![cat-file-io-badge]][cat-file-io] |
| [Read and write integers in little-endian byte order][ex-byteorder-le] | [![byteorder-badge]][byteorder] | [![cat-serialization-badge]][cat-serialization] |
| [Generate random floating point numbers][ex-rand-float] | [![rand-badge]][rand] | [![cat-math-badge]][cat-math] |
| [Generate random values on a custom type][ex-rand-custom] | [![rand-badge]][rand] | [![cat-math-badge]][cat-math] |
| [Construct a graph of objects][ex-petgraph-basic] | [![petgraph-badge]][petgraph] | [![cat-math-badge]][cat-math] |

## Serialization

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Serialize and deserialize unstructured JSON][ex-json-basic] | [![json-badge]][json] | [![cat-serialization-badge]][cat-serialization] |
| [Deserialize an unstructured TOML configuration file][ex-toml-basic] | [![toml-badge]][toml] | [![cat-serialization-badge]][cat-serialization] |

## Concurrency

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Mutate the elements of an array in parallel][ex-rayon-iter-mut] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |

## Application development

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Parse command line arguments][ex-clap-basic] | [![clap-badge]][clap] | [![cat-app-badge]][cat-app] |

<!--

Links, in a few categories. Follow the existing structure.

Individual pages contain a subset of these exact links, depending on
the crates and categories of their examples.

Keep lines sorted.

-->

<!- Categories -->

[cat-app-badge]: https://img.shields.io/badge/-app-orange.svg
[cat-app]: https://crates.io
[cat-concurrency-badge]: https://img.shields.io/badge/-concurrency-orange.svg
[cat-concurrency]: https://crates.io
[cat-file-io-badge]: https://img.shields.io/badge/-file_io-orange.svg
[cat-file-io]: https://crates.io
[cat-math-badge]: https://img.shields.io/badge/-rand-orange.svg
[cat-math]: https://crates.io
[cat-serialization-badge]: https://img.shields.io/badge/-serialization-orange.svg
[cat-serialization]: https://crates.io

<!-- Crates -->

[byteorder-badge]: https://img.shields.io/badge/byteorder-1.0.0-blue.svg
[byteorder]: https://docs.rs/byteorder/1.0.0/byteorder/
[clap-badge]: https://img.shields.io/badge/clap-2.22.2-blue.svg
[clap]: https://docs.rs/clap/2.22.0/rand/
[json-badge]: https://img.shields.io/badge/json-0.11.5-blue.svg
[json]: https://docs.rs/json/0.11.5/json/
[petgraph-badge]: https://img.shields.io/badge/petgraph-0.4.3-blue.svg
[petgraph]: https://docs.rs/petgraph/0.4.3/petgraph/
[rand-badge]: https://img.shields.io/badge/rand-0.3.15-blue.svg
[rand]: https://docs.rs/rand/0.3.15/rand/
[rayon-badge]: https://img.shields.io/badge/rayon-0.6.0-blue.svg
[rayon]: https://docs.rs/rayon/0.6.0/rayon/
[std-badge]: https://img.shields.io/badge/std-1.17.0-blue.svg
[std]: https://doc.rust-lang.org/std
[toml-badge]: https://img.shields.io/badge/toml-0.3.0-blue.svg
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
