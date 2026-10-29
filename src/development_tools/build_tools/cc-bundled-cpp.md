### Compile and link statically to a bundled C++ library

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

Linking a bundled C++ library is very similar to linking a bundled C library. The two core differences when compiling and statically linking a bundled C++ library are specifying a C++ compiler via the builder method [`cpp(true)`][cc-build-cpp] and preventing name mangling by the C++ compiler by adding the `extern "C"` section at the top of our C++ source file.


`build.rs`

```rust
{{#include ../../../crates/development_tools/build_tools/cc_bundled_cpp/build.rs }}
```

`src/foo.cpp`

```cpp
{{#include ../../../crates/development_tools/build_tools/cc_bundled_cpp/src/foo.cpp }}
```

`src/main.rs`

```rust
{{#include ../../../crates/development_tools/build_tools/cc_bundled_cpp/src/main.rs }}
```

[cc-build-cpp]: https://docs.rs/cc/*/cc/struct.Build.html#method.cpp
