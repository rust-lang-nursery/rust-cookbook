## モジュールごとのログレベルを許可する

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

[`RUST_LOG`]環境変数で個別に制御されるロギングディレクティブを使用して、`foo`とネストされた`foo::bar`の2つのモジュールを作成します。

```rust
#[macro_use]
extern crate log;
extern crate env_logger;

mod foo {
    mod bar {
        pub fn run() {
            warn!("[bar] warn");
            info!("[bar] info");
            debug!("[bar] debug");
        }
    }

    pub fn run() {
        warn!("[foo] warn");
        info!("[foo] info");
        debug!("[foo] debug");
        bar::run();
    }
}

fn main() {
    env_logger::init();
    warn!("[root] warn");
    info!("[root] info");
    debug!("[root] debug");
    foo::run();
}
```

[`RUST_LOG`]環境変数は[`env_logger`][env_logger]出力をコントロールします。
モジュールをカンマ区切りで`path::to::module=log_level`のような形式で宣言します。次のテストアプリを実行しましょう。
```bash
RUST_LOG="warn,test::foo=info,test::foo::bar=debug" ./test
```

デフォルトの`path::to::module=log_level`を`warn`に設定し、`foo`モジュールと`foo::bar`モジュールを`info`, `debug`にする。
```bash
WARN:test: [root] warn
WARN:test::foo: [foo] warn
INFO:test::foo: [foo] info
WARN:test::foo::bar: [bar] warn
INFO:test::foo::bar: [bar] info
DEBUG:test::foo::bar: [bar] debug
```

[`log::Level`]: https://docs.rs/log/*/log/enum.Level.html
[`RUST_LOG`]: https://docs.rs/env_logger/*/env_logger/#enabling-logging
