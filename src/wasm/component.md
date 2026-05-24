# Component Model — typed strings and structs

[![wasmtime-badge]][wasmtime] [![wasmtime-wasi-badge]][wasmtime-wasi] [![wit-bindgen-badge]][wit-bindgen] [![cat-wasm-badge]][cat-wasm]

Core WebAssembly function parameters are limited to `i32`, `i64`, `f32`, and
`f64`. The [WebAssembly Component Model][cm-spec] removes that restriction by
adding a language-neutral interface description language called
[WIT][wit-spec]. Strings, lists, records, and variants all become first-class
types that cross the guest/host boundary without manual pointer arithmetic.

[cm-spec]: https://component-model.bytecodealliance.org/
[wit-spec]: https://component-model.bytecodealliance.org/design/wit.html

This recipe requires two extra tools:

```shell
cargo install cargo-component
rustup target add wasm32-wasip2
```

## Define the interface in WIT

A WIT *world* declares what a component exports and imports. This one exports a
single `greet` function that takes and returns a `string`.

```wit
{{#include ../../crates/wasm_component_guest/wit/world.wit}}
```

## Implement the guest

[`wit_bindgen::generate!`][wit-bindgen-generate] reads the WIT file at compile
time and emits a `Guest` trait. The concrete type `Component` implements it,
and [`export!`][wit-bindgen-export] wires it up as the component's entry point.

[wit-bindgen-generate]: https://docs.rs/wit-bindgen/latest/wit_bindgen/macro.generate.html
[wit-bindgen-export]: https://docs.rs/wit-bindgen/latest/wit_bindgen/macro.generate.html#exports-the-export-macro

```rust,ignore
{{#include ../../crates/wasm_component_guest/src/lib.rs}}
```

Build the guest into a `.wasm` component:

```shell
cd crates/wasm_component_guest
cargo component build --release --target wasm32-wasip2
```

## Run it from the host

The host uses [`wasmtime::component::bindgen!`][bindgen-macro] — the same WIT
file, read at compile time — to generate a typed `Greeter` wrapper. [`Config`]
enables the component model, [`wasmtime_wasi::p2::add_to_linker_sync`] wires
up WASI Preview 2 (required even for guests that don't use WASI directly), and
[`Greeter::instantiate`] produces a handle whose `call_greet` method accepts a
`&str` and returns a `String`.

[bindgen-macro]: https://docs.rs/wasmtime/*/wasmtime/component/macro.bindgen.html
[`Config`]: https://docs.rs/wasmtime/*/wasmtime/struct.Config.html
[`wasmtime_wasi::p2::add_to_linker_sync`]: https://docs.rs/wasmtime-wasi/*/wasmtime_wasi/p2/fn.add_to_linker_sync.html
[`Greeter::instantiate`]: https://docs.rs/wasmtime/*/wasmtime/component/struct.Instance.html

```rust,ignore
{{#include ../../crates/wasm_component_host/src/main.rs}}
```

Run from the repo root (after building the guest):

```shell
cargo run --manifest-path crates/wasm_component_host/Cargo.toml
# Hello, WebAssembly!
```

{{#include ../links.md}}
