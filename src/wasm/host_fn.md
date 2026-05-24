# Define host functions for WebAssembly

[![wasmtime-badge]][wasmtime] [![cat-wasm-badge]][cat-wasm]

WebAssembly modules can import functions from the host, enabling guests to
call back into Rust. This recipe defines a `print` function in the host that
the WASM guest calls to write a message stored in its own linear memory.

[`Func::wrap`][func-wrap] creates a host-defined function from a Rust closure.
The first parameter is a [`Caller`][caller] — a handle to the calling
instance's store. The guest module declares the import with
`(import "host" "print" ...)` and the host passes `[print_fn.into()]` as the
imports slice to [`Instance::new`][instance-new].

[`Instance::new`][instance-new] takes `&[Extern]` — the enum that can hold a
[`Func`][func-type], `Memory`, `Global`, or `Table`. `.into()` converts the
`Func` into an `Extern` so the slice type is satisfied.

Inside the closure, [`Caller::get_export`][get-export] retrieves the guest's
`memory` export so the host can read the bytes the guest passed by pointer and
length.

[func-type]: https://docs.rs/wasmtime/*/wasmtime/struct.Func.html

[func-wrap]: https://docs.rs/wasmtime/*/wasmtime/struct.Func.html#method.wrap
[caller]: https://docs.rs/wasmtime/*/wasmtime/struct.Caller.html
[instance-new]: https://docs.rs/wasmtime/*/wasmtime/struct.Instance.html#method.new
[get-export]: https://docs.rs/wasmtime/*/wasmtime/struct.Caller.html#method.get_export

```rust
{{#include ../../crates/wasm/src/bin/host_fn.rs::38}}
```

Run it:

```shell
cargo run --manifest-path crates/wasm/Cargo.toml --bin host_fn
# [wasm] Hello from WebAssembly!
```

{{#include ../links.md}}
