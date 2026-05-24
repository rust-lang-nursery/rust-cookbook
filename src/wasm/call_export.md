# Call an exported WebAssembly function

[![wasmtime-badge]][wasmtime] [![cat-wasm-badge]][cat-wasm]

[`wasmtime`][wasmtime] embeds a WebAssembly runtime inside a Rust host
program. This recipe compiles a module from [WebAssembly Text format
(WAT)][wat-spec], instantiates it, and calls an exported function.

The three core types are [`Engine`][engine], which holds compiler settings;
[`Module`][module], which holds compiled bytecode; and [`Store`][store], which
holds all runtime state for a set of instances.

[`Instance::new`][instance-new] links the compiled module with its imports
(none here) and produces a live instance. [`get_typed_func`][get-typed-func]
looks up an export by name and checks that its signature matches the Rust
generic parameters — `(i32, i32)` in, `i32` out.

Core WebAssembly function parameters are limited to four numeric types:
`i32`, `i64`, `f32`, and `f64`. Strings, structs, and other complex values
cannot cross the function boundary directly — see [Exchange data via linear
memory][ex-wasm-linear-memory] for the pointer-and-length pattern, or the
[Component Model][wasm-component-model] for a higher-level IDL that lifts
these restrictions.

[ex-wasm-linear-memory]: linear_memory.html#exchange-data-via-webassembly-linear-memory
[wasm-component-model]: https://component-model.bytecodealliance.org/

[engine]: https://docs.rs/wasmtime/*/wasmtime/struct.Engine.html
[module]: https://docs.rs/wasmtime/*/wasmtime/struct.Module.html
[store]: https://docs.rs/wasmtime/*/wasmtime/struct.Store.html
[instance-new]: https://docs.rs/wasmtime/*/wasmtime/struct.Instance.html#method.new
[get-typed-func]: https://docs.rs/wasmtime/*/wasmtime/struct.Instance.html#method.get_typed_func
[wat-spec]: https://webassembly.github.io/spec/core/text/index.html

```rust
{{#include ../../crates/wasm/src/bin/call_export.rs::23}}
```

Run it:

```shell
cargo run --manifest-path crates/wasm/Cargo.toml --bin call_export
# 5 + 37 = 42
```

{{#include ../links.md}}
