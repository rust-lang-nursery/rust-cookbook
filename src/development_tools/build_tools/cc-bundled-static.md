## Compile and link statically to a bundled C library

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

To accommodate scenarios where additional C, C++, or assembly is required in a project, the [**cc**][cc] crate
offers a simple api for compiling bundled C/C++/asm code into static libraries (**.a**) that can be statically linked to by **rustc**.

The following example has some bundled C code (**src/hello.c**) that will be used from rust.
Before compiling rust source code, the "build" file (**build.rs**) specified in **Cargo.toml** runs.
Using the [**cc**][cc] crate, a static library file will be produced (in this case, **libhello.a**, see
[`compile` docs][cc-build-compile]) which can then be used from rust by declaring the external function signatures in an `extern` block.

Since the bundled C is very simple, only a single source file needs to be passed to [`cc::Build`][cc-build].
For more complex build requirements, [`cc::Build`][cc-build] offers a full suite of builder methods for specifying
[`include`][cc-build-include] paths and extra compiler [`flag`][cc-build-flag]s.

### `Cargo.toml`

```toml
[package]
...
build = "build.rs"

[build-dependencies]
cc = "1"

[dependencies]
error-chain = "0.11"
```

### `build.rs`

```rust,no_run
extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");   // outputs `libhello.a`
}
```

### `src/hello.c`

```c
#include <stdio.h>


void hello() {
    printf("Hello from C!\n");
}

void greet(const char* name) {
    printf("Hello, %s!\n", name);
}
```

### `src/main.rs`

```rust,ignore
# #[macro_use] extern crate error_chain;
use std::ffi::CString;
use std::os::raw::c_char;
#
# error_chain! {
#     foreign_links {
#         NulError(::std::ffi::NulError);
#         Io(::std::io::Error);
#     }
# }
#
# fn prompt(s: &str) -> Result<String> {
#     use std::io::Write;
#     print!("{}", s);
#     std::io::stdout().flush()?;
#     let mut input = String::new();
#     std::io::stdin().read_line(&mut input)?;
#     Ok(input.trim().to_string())
# }

extern {
    fn hello();
    fn greet(name: *const c_char);
}

fn run() -> Result<()> {
    unsafe { hello() }
    let name = prompt("What's your name? ")?;
    let c_name = CString::new(name)?;
    unsafe { greet(c_name.as_ptr()) }
    Ok(())
}
#
# quick_main!(run);
```

[`cc::Build::define`]: https://docs.rs/cc/*/cc/struct.Build.html#method.define
[`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
[cc-build-compile]: https://docs.rs/cc/*/cc/struct.Build.html#method.compile
[cc-build-cpp]: https://docs.rs/cc/*/cc/struct.Build.html#method.cpp
[cc-build-flag]: https://docs.rs/cc/*/cc/struct.Build.html#method.flag
[cc-build-include]: https://docs.rs/cc/*/cc/struct.Build.html#method.include
[cc-build]: https://docs.rs/cc/*/cc/struct.Build.html
