## Compile and link statically to an external C library

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

Often the C code you need is not yours: it is a third-party library whose source you vendor into your project,
with its own layout of public headers (**include/**) and implementation files (**src/**).
The [**cc**][cc] crate can compile such a library from **build.rs** into a static archive (**libfnv.a**) that **rustc** links into the final binary,
so the resulting executable has no runtime dependency on the library.

The following recipe vendors a tiny hashing library (a 64-bit FNV-1a implementation) in **vendor/fnv**.
[`cc::Build::include`] adds the library's header directory to the compiler's search path, and [`cc::Build::file`] adds its source.
Because third-party code is not ours to fix, [`cc::Build::warnings`] is set to `false` so the library's warnings do not clutter the build output.
[`cc::Build::try_compile`] returns a `Result` instead of panicking and also emits the `cargo:rustc-link-lib=static=fnv`
and `cargo:rustc-link-search` directives, so no manual linking instructions are needed.
The `cargo:rerun-if-changed` line re-runs the build script when the vendored library changes.

The Rust side declares the function's signature in an `extern` block, as in the
[bundled C library recipe](build_tools.html#compile-and-link-statically-to-a-bundled-c-library), and wraps the `unsafe` call in a safe function.

To copy this recipe, copy the whole **cc_external_static** crate into your project and replace **vendor/fnv** with your own library.

`build.rs`

```rust
{{#include ../../../crates/development_tools/build_tools/cc_external_static/build.rs }}
```

`vendor/fnv/include/fnv.h`

```c
{{#include ../../../crates/development_tools/build_tools/cc_external_static/vendor/fnv/include/fnv.h }}
```

`vendor/fnv/src/fnv.c`

```c
{{#include ../../../crates/development_tools/build_tools/cc_external_static/vendor/fnv/src/fnv.c }}
```

`src/main.rs`

```rust
{{#include ../../../crates/development_tools/build_tools/cc_external_static/src/main.rs }}
```

[`cc::Build::file`]: https://docs.rs/cc/*/cc/struct.Build.html#method.file
[`cc::Build::include`]: https://docs.rs/cc/*/cc/struct.Build.html#method.include
[`cc::Build::try_compile`]: https://docs.rs/cc/*/cc/struct.Build.html#method.try_compile
[`cc::Build::warnings`]: https://docs.rs/cc/*/cc/struct.Build.html#method.warnings
