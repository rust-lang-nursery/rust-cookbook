## カスタム定義が設定されている時、Cライブラリをコンパイルする

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

[`cc::Build::define`]によるカスタム定義でバンドルされたCコードをビルドすることは簡単です。
このメソッドは[`Option`]値を受け取るため`#define APP_NAME "foo"`や`#define WELCOME`のような定義をすることができます(値のない定義には`None`を与える)。この例では実行時に`build.rs`にセットされた動的な定義を含むバンドルされたCファイルで"**Welcome to foo - version 1.0.2**"と出力します。Cargoはいくつかの[environment variables][cargo-env]を設定します。これはカスタム定義に役立つかもしれません。

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
