### Compile a C library while setting custom defines

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

It is simple to build bundled C code with custom defines using [`cc::Build::define`].
The method takes an [`Option`] value, so it is possible to create defines such as `#define APP_NAME "foo"`
as well as `#define WELCOME` (pass `None` as the value for a value-less define). This recipe builds
a bundled C file with dynamic defines set in `build.rs` and prints "**Welcome to foo - version 1.0.2**"
when run. Cargo sets some [environment variables][cargo-env] which may be useful for some custom defines.


`build.rs`

```rust
{{#include ../../../crates/development_tools/build_tools/cc_defines/build.rs }}
```

`src/foo.c`

```c
{{#include ../../../crates/development_tools/build_tools/cc_defines/src/foo.c }}
```

`src/main.rs`

```rust
{{#include ../../../crates/development_tools/build_tools/cc_defines/src/main.rs }}
```

[cargo-env]: https://doc.rust-lang.org/cargo/reference/environment-variables.html
