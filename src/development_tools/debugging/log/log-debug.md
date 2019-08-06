## コンソールにデバッグログメッセージを表示する

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

The `log` crate provides logging utilities. The `env_logger` crate configures
logging via an environment variable.  The [`debug!`] macro works like other
[`std::fmt`] formatted strings.

`log`クレートはloggingユーティリティを提供します。`env_logger`は環境変数からloggingの設定をします。[`debug!`]マクロは文字列をフォーマットする[`std::fmt`]と同様な機能を持ちます。

```rust
#[macro_use]
extern crate log;
extern crate env_logger;

fn execute_query(query: &str) {
    debug!("Executing query: {}", query);
}

fn main() {
    env_logger::init();

    execute_query("DROP TABLE students");
}
```

デフォルトのログレベルは`error`で低レベルなものは出力されない。なのでこのコードを実行した時はなにも出力がされない。

メッセージを出力するために環境変数`RUST_LOG`を設定する:
```
$ RUST_LOG=debug cargo run
```

Cargoは出力の最後に次のようなデバッグ情報を表示する。
```
DEBUG:main: Executing query: DROP TABLE students
```

[`debug!`]: https://docs.rs/log/*/log/macro.debug.html
[`std::fmt`]: https://doc.rust-lang.org/std/fmt/
