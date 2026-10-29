### Compile and link statically to a bundled C library

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

To accommodate scenarios where additional C, C++, or assembly is required in a project, the [**cc**][cc] crate
offers a simple api for compiling bundled C/C++/asm code into static libraries (**.a**) that can be statically linked to by **rustc**.

The following recipe has some bundled C code (**src/hello.c**) that will be used from rust.
Before compiling rust source code, the "build" file (**build.rs**) specified in **Cargo.toml** runs.
Using the [**cc**][cc] crate, a static library file will be produced (in this case, **libhello.a**, see
[`compile` docs][cc-build-compile]) which can then be used from rust by declaring the external function signatures in an `extern` block.

Since the bundled C is very simple, only a single source file needs to be passed to [`cc::Build`][cc-build].
For more complex build requirements, [`cc::Build`][cc-build] offers a full suite of builder methods for specifying
[`include`][cc-build-include] paths and extra compiler [`flag`][cc-build-flag]s.

`build.rs`

```rust
{{#include ../../../crates/development_tools/build_tools/cc_bundled_static/build.rs }}
```

`src/hello.c`

```c
{{#include ../../../crates/development_tools/build_tools/cc_bundled_static/src/hello.c }}
```

`src/main.rs`

```rust
{{#include ../../../crates/development_tools/build_tools/cc_bundled_static/src/main.rs }}
```

[`cc::Build::define`]: https://docs.rs/cc/*/cc/struct.Build.html#method.define
[`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
[cc-build-compile]: https://docs.rs/cc/*/cc/struct.Build.html#method.compile
[cc-build-cpp]: https://docs.rs/cc/*/cc/struct.Build.html#method.cpp
[cc-build-flag]: https://docs.rs/cc/*/cc/struct.Build.html#method.flag
[cc-build-include]: https://docs.rs/cc/*/cc/struct.Build.html#method.include
[cc-build]: https://docs.rs/cc/*/cc/struct.Build.html
