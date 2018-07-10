## Compile a C library while setting custom defines

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

It is simple to build bundled C code with custom defines using [`cc::Build::define`].
The method takes an [`Option`] value, so it is possible to create defines such as `#define APP_NAME "foo"`
as well as `#define WELCOME` (pass `None` as the value for a value-less define). This example builds
a bundled C file with dynamic defines set in `build.rs` and prints "**Welcome to foo - version 1.0.2**"
when run. Cargo sets some [environment variables][cargo-env] which may be useful for some custom defines.


### `Cargo.toml`

```toml
[package]
...
version = "1.0.2"
build = "build.rs"

[build-dependencies]
cc = "1"
```

### `build.rs`

```rust,no_run
extern crate cc;

fn main() {
    cc::Build::new()
        .define("APP_NAME", "\"foo\"")
        .define("VERSION", format!("\"{}\"", env!("CARGO_PKG_VERSION")).as_str())
        .define("WELCOME", None)
        .file("src/foo.c")
        .compile("foo");
}
```

### `src/foo.c`

```c
#include <stdio.h>

void print_app_info() {
#ifdef WELCOME
    printf("Welcome to ");
#endif
    printf("%s - version %s\n", APP_NAME, VERSION);
}
```

### `src/main.rs`

```rust,ignore
extern {
    fn print_app_info();
}

fn main(){
    unsafe {
        print_app_info();
    }   
}
```

[cargo-env]: https://doc.rust-lang.org/cargo/reference/environment-variables.html
