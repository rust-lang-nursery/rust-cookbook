## stdoutを使ったログの表示

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

[`Builder::target`]でカスタムロガーの設定を作り、ログ出力のターゲットを[`Target::Stdout`]に設定します。

```rust
#[macro_use]
extern crate log;
extern crate env_logger;

use env_logger::{Builder, Target};

fn main() {
    Builder::new()
        .target(Target::Stdout)
        .init();

    error!("This error has been printed to Stdout");
}
```

[`Builder::target`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html#method.target
[`Target::Stdout`]: https://docs.rs/env_logger/*/env_logger/fmt/enum.Target.html
