## ロギングのセットアップにカスタム環境戦数を使う

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

[`Builder`]はloggingの設定を行います。

[`Builder::parse`]は[`RUST_LOG`]シンタックスの環境変数`MY_APP_LOG`をパースして、[`Builder::init`]でloggerを初期化します。これらの全てのステップは通常、[`Builder::init`]により内部で終了しています。

```rust
#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use env_logger::Builder;

fn main() {
    Builder::new()
        .parse(&env::var("MY_APP_LOG").unwrap_or_default())
        .init();

    info!("informational message");
    warn!("warning message");
    error!("this is an error {}", "message");
}
```

[`env_logger::init`]: https://docs.rs/env_logger/*/env_logger/fn.init.html
[`Builder`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html
[`Builder::init`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html#method.init
[`Builder::parse`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html#method.parse
[`RUST_LOG`]: https://docs.rs/env_logger/*/env_logger/#enabling-logging
