# WebAssembly

[`wasmtime`][wasmtime] is an embeddable WebAssembly runtime. These recipes
cover the **host embedding API** — loading modules, calling exports, sharing
linear memory, and wiring up host-defined functions that guests can call back
into.

The recipes live outside the main workspace because `wasmtime` is a large
dependency not used elsewhere. Run them with
`cargo run --manifest-path crates/wasm/Cargo.toml --bin <name>`.

**Embedding wasmtime**

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Call an exported WebAssembly function][ex-wasm-call-export] | [![wasmtime-badge]][wasmtime] | [![cat-wasm-badge]][cat-wasm] |
| [Exchange data via WebAssembly linear memory][ex-wasm-linear-memory] | [![wasmtime-badge]][wasmtime] | [![cat-wasm-badge]][cat-wasm] |
| [Define host functions for WebAssembly][ex-wasm-host-fn] | [![wasmtime-badge]][wasmtime] | [![cat-wasm-badge]][cat-wasm] |
| [Component Model — typed strings and structs][ex-wasm-component] | [![wasmtime-badge]][wasmtime] [![wasmtime-wasi-badge]][wasmtime-wasi] [![wit-bindgen-badge]][wit-bindgen] | [![cat-wasm-badge]][cat-wasm] |

[ex-wasm-call-export]: wasm/call_export.html#call-an-exported-webassembly-function
[ex-wasm-linear-memory]: wasm/linear_memory.html#exchange-data-via-webassembly-linear-memory
[ex-wasm-host-fn]: wasm/host_fn.html#define-host-functions-for-webassembly
[ex-wasm-component]: wasm/component.html#component-model--typed-strings-and-structs

{{#include links.md}}
