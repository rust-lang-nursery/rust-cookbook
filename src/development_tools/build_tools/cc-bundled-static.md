## バンドルされたCライブラリをコンパイルし静的にリンクする

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

追加のC, C++, アセンブリがプロジェクトに必要な場合に対応するため、[**cc**][cc]クレートは**rustc**によってバンドルされた静的リンク可能なC/C++/アセンブリコードを静的ライブラリにコンパイルするためのシンプルなapiを提供しています。

下の例はrustから使ういくつかのバンドル済みCコード(**src/hello.c**)があります。
rustコードにコンパイルする前に**Cargo.toml**で指定された"ビルド"ファイル(**build.rs**) が実行されます。
[**cc**][cc]クレートを使い、静的ライブラリファイルが作られ(この例では**libhello.a**です。[`compile` docs][cc-build-compile]を参照)、`extern`ブロックで外部関数シグネチャを宣言することによってrustから使用することができます。

バンドル済みCはとても簡単で、単一ソースファイルは[`cc::Build`][cc-build]に渡す必要があります。より複雑なビルドは[`include`][cc-build-include]パスと追加のコンパイラ[`フラグ`]を指定するためのビルダーメソッドが[`cc::Build`][cc-build]に用意されています。

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
