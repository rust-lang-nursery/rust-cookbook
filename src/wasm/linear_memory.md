# Exchange data via WebAssembly linear memory

[![wasmtime-badge]][wasmtime] [![cat-wasm-badge]][cat-wasm]

WebAssembly modules communicate with their host through [linear
memory][linear-memory-spec] — a flat, byte-addressable array that both sides
can read and write. This recipe writes two `i32` values into the WASM module's
memory, calls a function that adds them, and reads the result back from the
return value.

[`get_memory`][get-memory] retrieves the module's exported `memory` object.
[`data_mut`][data-mut] gives a mutable byte slice covering the entire linear
memory so the host can write inputs before the call.

[linear-memory-spec]: https://webassembly.github.io/spec/core/exec/runtime.html#memory-instances
[get-memory]: https://docs.rs/wasmtime/*/wasmtime/struct.Instance.html#method.get_memory
[data-mut]: https://docs.rs/wasmtime/*/wasmtime/struct.Memory.html#method.data_mut

```rust
{{#include ../../crates/wasm/src/bin/linear_memory.rs::33}}
```

Run it:

```shell
cargo run --manifest-path crates/wasm/Cargo.toml --bin linear_memory
# 17 + 25 = 42
```

{{#include ../links.md}}
