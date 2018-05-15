# Build Time Tooling

This section covers "build-time" tooling, or code that is run prior to compiling a crate's source code.
Conventionally, build-time code lives in a **build.rs** file and is commonly referred to as a "build script".
Common use cases include rust code generation and compilation of bundled C/C++/asm code.
See crates.io's [documentation on the matter][build-script-docs] for more information.




{{#include build_tools/cc-bundled-static.md}}



{{#include build_tools/cc-bundled-cpp.md}}

{{#include build_tools/cc-defines.md}}

{{#include ../links.md}}

<!-- Other Reference -->

[`cc::Build::define`]: https://docs.rs/cc/*/cc/struct.Build.html#method.define
[`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
[build-script-docs]: http://doc.crates.io/build-script.html
[cargo-env]: http://doc.crates.io/environment-variables.html#environment-variables-cargo-sets-for-crates
[cc-build-compile]: https://docs.rs/cc/*/cc/struct.Build.html#method.compile
[cc-build-cpp]: https://docs.rs/cc/*/cc/struct.Build.html#method.cpp
[cc-build-flag]: https://docs.rs/cc/*/cc/struct.Build.html#method.flag
[cc-build-include]: https://docs.rs/cc/*/cc/struct.Build.html#method.include
[cc-build]: https://docs.rs/cc/*/cc/struct.Build.html
[playground]: https://play.rust-lang.org
