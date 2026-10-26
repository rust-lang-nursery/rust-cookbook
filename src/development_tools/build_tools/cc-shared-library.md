## Compile and link dynamically to a C++ shared library

[![std-badge]][std] [![cat-development-tools-badge]][cat-development-tools]

For more complex scenarios, a rust project may need to integrate with a shared
library that is dynamically linked at runtime instead of bundled into the
binary. This is useful when embedding a system-level library that isn't
intended to be compiled into client applications directly, or a library that
gets updated regularly outside the scope of your project. A mixture of build
scripts (`build.rs`) and compiler directives teaches the compiler, and the
final binary, how to find and link to the shared library.

This example does not use the [`cc`][cc] crate to compile the shared library,
since you often don't have access to a shared library's source or build
process — you're just consuming its binary directly. For the purpose of this
example, the shared library's source is in `src/mylibrary.h` and
`src/mylibrary.cc` so its API surface is visible, and a `Makefile` builds it
the way you would outside of the rust project's own build process, independent
of the [`cc`][cc] crate.

Before compiling rust source code, the "build" file (`build.rs`) specified in
`Cargo.toml` runs. It builds `libmylibrary.so` by invoking `make`, tells
**rustc** where to find it at compile time with the
[`cargo:rustc-link-search`][rustc-link-search] directive (pointed at Cargo's
own `OUT_DIR`, rather than a fixed path relative to the crate root), and
enumerates the shared library dependency with the
[`cargo:rustc-link-lib`][rustc-link-lib] directive.

[`cargo:rustc-link-lib`][rustc-link-lib] also bakes dynamic dependency metadata
into the final binary, which tells the linker the name of the library to find
at runtime. At runtime it would normally search default system locations
(e.g. `/usr/lib`), which won't contain `libmylibrary.so` since it was built
into a Cargo-managed output directory. The [`cargo:rustc-link-arg`][rustc-link-arg]
directive bakes a runtime search path (an "rpath") into the binary pointing at
that `OUT_DIR`, so the example runs without manually setting
`LD_LIBRARY_PATH`.

`build.rs`

```rust
{{#include ../../../crates/development_tools/build_tools/cc_shared_library/build.rs }}
```

`Makefile`

```makefile
{{#include ../../../crates/development_tools/build_tools/cc_shared_library/Makefile }}
```

`src/mylibrary.h`

```cpp
{{#include ../../../crates/development_tools/build_tools/cc_shared_library/src/mylibrary.h }}
```

`src/mylibrary.cc`

```cpp
{{#include ../../../crates/development_tools/build_tools/cc_shared_library/src/mylibrary.cc }}
```

`src/main.rs`

```rust
{{#include ../../../crates/development_tools/build_tools/cc_shared_library/src/main.rs }}
```

[cc]: https://docs.rs/cc/latest/cc/
[rustc-link-arg]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg
[rustc-link-lib]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib
[rustc-link-search]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search
