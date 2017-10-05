# Build Time Tooling

This section covers "build-time" tooling, or code that is run prior to compiling a crate's source code.
Conventionally, build-time code lives in a **build.rs** file and is commonly referred to as a "build script".
Common use cases include rust code generation and compilation of bundled C/C++/asm code.
See crates.io's [documentation on the matter][build-script-docs] for more information.


| Recipe | Crates | Categories |
|--------|--------|------------|
| [Compile and link statically to a bundled C library][ex-cc-static-bundled] | [![cc-badge]][cc] | [![cat-development-tools-badge]][cat-development-tools] |
| [Compile and link statically to a bundled C++ library][ex-cc-static-bundled-cpp] | [![cc-badge]][cc] | [![cat-development-tools-badge]][cat-development-tools] |


[ex-cc-static-bundled]: #ex-cc-static-bundled
<a name="ex-cc-static-bundled"></a>
## Compile and link statically to a bundled C library

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

To accommodate scenarios where additional C, C++, or assembly is required in a project, the [**cc**][cc] crate
offers a simple api for compiling bundled C/C++/asm code into static libraries (**.a**) that can be statically linked to by **rustc**.

The following example has some bundled C code (**src/hello.c**) that will be used from rust.
Before compiling our rust source code, the "build" file (**build.rs**) specified in **Cargo.toml** will run.
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



[ex-cc-static-bundled-cpp]: #ex-cc-static-bundled-cpp
<a name="ex-cc-static-bundled-cpp"></a>
## Compile and link statically to a bundled C++ library

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

Linking a bundled C++ library is almost the same as linking it with a C library. The main difference is the [`cpp`][cc-build-cpp] option that should be put to `true` and the addition of `extern "C"` in the foo.cpp file.


### `Cargo.toml`

```toml
[package]
...
build = "build.rs"

[build-dependencies]
cc = "1"
```

### `build.rs`

```rust,no_run
extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/foo.cpp")
        .compile("foo");   
}
```

### `src/foo.cpp`

```cpp
extern "C" {
    int multiply(int x, int y);
}

int multiply() {
    return x*y;
}
```

### `src/main.rs`

```rust,ignore
extern {
    fn multiply(x : i32, y : i32);
}

fn main(){
    unsafe {
        println!("{}", multiply(5,7));
    }   
}
```

{{#include links.md}}

<!-- Other Reference -->

[build-script-docs]: http://doc.crates.io/build-script.html
[playground]: https://play.rust-lang.org
[cc-build]: https://docs.rs/cc/*/cc/struct.Build.html
[cc-build-include]: https://docs.rs/cc/*/cc/struct.Build.html#method.include
[cc-build-flag]: https://docs.rs/cc/*/cc/struct.Build.html#method.flag
[cc-build-compile]: https://docs.rs/cc/*/cc/struct.Build.html#method.compile
[cc-build-cpp]: https://docs.rs/cc/*/cc/struct.Build.html#method.cpp