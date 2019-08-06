## バンドルされたC++ライブラリをコンパイルし静的にリンクする

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

バンドルされたC++ライブラリをリンクすることはバンドルされたCライブラリをリンクすることに似ています。二つの主な違いは、バンドルしたC++ライブラリをコンパイルと静的リンクする時にビルダーメソッド[`cpp(true)`][cc-build-cpp]からC++コンパイラを指定し、C++ソースファイルの先頭に`extern "C"`を追加することです。

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

int multiply(int x, int y) {
    return x*y;
}
```

### `src/main.rs`

```rust,ignore
extern {
    fn multiply(x : i32, y : i32) -> i32;
}

fn main(){
    unsafe {
        println!("{}", multiply(5,7));
    }   
}
```

[cc-build-cpp]: https://docs.rs/cc/*/cc/struct.Build.html#method.cpp
