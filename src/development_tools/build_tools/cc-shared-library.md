## Compile and link statically to a bundled C library

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

To accommodate more complex scenarios where your rust project needs to integrate
with a shared library that will be dynamically linked at runtime, we can use a
mixture of build scripts (**build.rs**) and compiler directives to teach the
compiler and the final binary how to find and link to the shared library. This
is useful when embedding a system-level library that aren't intended to be
compiled into their client applications directly, or a library that gets updated
regularly outside the scope of your project.

This example does not use the [**cc**][cc] crate to compile the shared library,
since you often don't have access to a shared library's source or build
process. Rather, you're just consuming the library's binary directly.

For the purpose of this example, the shared library's source is in
**src/mylibrary.h** and **src/mylibrary.cc** so we can transparently see its API
surface. But you can build and distribute the resulting shared library binary
independent of your rust project's build process. This example also provides a
simple **src/Makefile** to show how you might build this simple library outside
of our our rust build process, to produce a suitable binary that our project can
consume.

Before compiling rust source code, the "build" file (**build.rs**) specified in
**Cargo.toml** runs. It enumerates the shared libraries that our project depends
on with the [`cargo:rustc-link-lib`][rustc-link-lib] directive, and tells
**rustc** how to find those libraries at compile time with the
[`cargo:rustc-link-search`][rustc-link-search] directive.

The [`cargo:rustc-link-lib`][rustc-link-lib] directive also bakes dynamic
dependency metadata into the final binary, which tells the linker the names of
the libraries to find at runtime. At runtime, it will search for these libraries
in its default places (usually system defaults like `/usr/lib`), but you can
also include specific runtime path information in the final binary with the
[`cargo:rustc-link-arg][rustc-link-arg] directive, which informs the linker
*where* to search for shared library dependencies. This can make your binary
more portable, but we do not do this in the simple example below.

### `Cargo.toml`

```toml
[package]
...
build = "build.rs"
```

### `build.rs`

```rust,edition2021,no_run
fn main() {
    // At compile time, we specify dynamic library dependency entries to bake
    // into the binary for the linker to consult at runtime.
    println!("cargo:rustc-link-search=native=.");
    println!("cargo:rustc-link-lib=dylib=mylibrary");
}
```

### `src/library.c`

```cpp
#include "api.h"

#include <iostream>

extern "C" {

int add_numbers(int a, int b) {
    return a + b;
}

void greet(const char* string) {
  std::cout << "Hello from a C shared library, " << string << std::endl;
}

}  // extern "C"
```

### `src/library.h`

```cpp
// Use C linkage for ABI stability.
extern "C" {

int add_numbers(int a, int b);
void greet(const char* string);

}  // extern "C"
```

### `src/Makefile`

```Makefile
CC=g++
CFLAGS=--std=c++11 -Wall -Werror -I.

all: lib

lib: src/mylibrary.c src/mylibrary.h
        # Position-independent code (`-fpic`) is required for shared libraries.
        $(CC) -c -fpic src/mylibrary.c -o mylibrary.o $(CFLAGS)
        $(CC) -shared -o libmylibrary.so mylibrary.o
        rm mylibrary.o
```

### `src/main.rs`

```rust,edition2021,ignore
use std::ffi::CString;
use std::os::raw::c_char;

// Compiler directive to tell the Rust which library contains the external
// symbols we enumerate in this specific `extern` block. Here we specify a C
// shared library called `libmylibrary`.
#[link(name = "mylibrary")]
extern "C" {
    fn add_numbers(a: i32, b: i32) -> i32;
    fn greet(name: *const c_char);
}

fn prompt(s: &str) -> String {
    use std::io::Write;
    print!("{}", s);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Safe wrapper around the unsafe C add_numbers function.
pub fn library_add_numbers(a: i32, b: i32) -> i32 {
    unsafe {
        add_numbers(a, b)
    }
}

// Safe wrapper around the unsafe C add_numbers function.
pub fn library_greet(name: CString) {
    unsafe {
        greet(name.as_ptr())
    }
}

fn main() {
    let name = prompt("What's your name? ");
    let c_name = CString::new(name).unwrap();
    library_greet(c_name);
    let result = library_add_numbers(10, 12);
    println!("Addition result: {}", result);
}
```

[cc]: https://docs.rs/cc/latest/cc/
[rustc-link-arg]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg
[rustc-link-lib]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib
[rustc-link-search]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search
