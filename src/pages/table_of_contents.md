# Cookin' with Rust

A practical guide to the Rust crate ecosystem.

## Recipes

### Foundations

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Read and write integers in little-endian byte order][ex-byteorder-le] | [![byteorder-badge]][byteorder] | [![cat-serialization-badge]][cat-serialization] |
| [Generate random floating point numbers][ex-rand-float] | [![rand-badge]][rand] | [![cat-math-badge]][cat-math] |
| [Generate random values on a custom type][ex-rand-custom] | [![rand-badge]][rand] | [![cat-math-badge]][cat-math] |
| [Construct a graph of objects][ex-petgraph-basic] | [![petgraph-badge]][petgraph] | [![cat-math-badge]][cat-math] |
| [Serialize and deserialize unstructured JSON][ex-json-basic] | [![json-badge]][json] | [![cat-serialization-badge]][cat-serialization] |
| [Deserialize an unstructured TOML configuration file][ex-toml-basic] | [![toml-badge]][toml] | [![cat-serialization-badge]][cat-serialization] |

### Concurrency

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Mutate the elements of an array in parallel][ex-rayon-iter-mut] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |

### I/O

### Application development

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Parse command line arguments][ex-clap-basic] | [![clap-badge]][clap] | [![cat-app-badge]][cat-app] |



## Contributing

If you'd like to make changes to the project, please see [this guide](pages/contributing.html).

## License

MIT/Apache-2.0

<!- Categories -->

[cat-serialization-badge]: https://img.shields.io/badge/-serialization-orange.svg
[cat-serialization]: https://crates.io
[cat-math-badge]: https://img.shields.io/badge/-rand-orange.svg
[cat-math]: https://crates.io
[cat-app-badge]: https://img.shields.io/badge/-app-orange.svg
[cat-app]: https://crates.io
[cat-concurrency-badge]: https://img.shields.io/badge/-concurrency-orange.svg
[cat-concurrency]: https://crates.io

<!-- Crates -->

[byteorder-badge]: https://img.shields.io/badge/byteorder-1.0.0-blue.svg
[byteorder]: https://docs.rs/byteorder/1.0.0/byteorder/
[petgraph-badge]: https://img.shields.io/badge/petgraph-0.4.3-blue.svg
[petgraph]: https://docs.rs/petgraph/0.4.3/petgraph/
[rand-badge]: https://img.shields.io/badge/rand-0.3.15-blue.svg
[rand]: https://docs.rs/rand/0.3.15/rand/
[clap-badge]: https://img.shields.io/badge/clap-2.22.2-blue.svg
[clap]: https://docs.rs/clap/2.22.0/rand/
[rayon-badge]: https://img.shields.io/badge/rayon-0.6.0-blue.svg
[rayon]: https://docs.rs/rayon/0.6.0/rayon/
[json-badge]: https://img.shields.io/badge/json-0.11.5-blue.svg
[json]: https://docs.rs/json/0.11.5/json/
[toml-badge]: https://img.shields.io/badge/toml-0.3.0-blue.svg
[toml]: https://docs.rs/toml/0.3.0/toml/

<!-- Examples -->

[ex-byteorder-le]: todo
[ex-rand-float]: todo
[ex-rand-custom]: todo
[ex-petgraph-basic]: todo
[ex-clap-basic]: todo
[ex-json-basic]: todo
[ex-toml-basic]: todo
[ex-rayon-iter-mut]: todo
