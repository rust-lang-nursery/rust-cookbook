## メインでエラーを正しく処理する

[![error-chain-badge]][error-chain] [![cat-rust-patterns-badge]][cat-rust-patterns]

[error-chain]を使ってファイルが存在しなかった時に起こるエラーのハンドリングをします。このライブラリは多くのボイラープレートが必要とする[Rustでのエラーハンドリング]を提供します。

[`foreign_links`]の中の`Io(std::io::Error)`は[`std::io::Error`]から[`error_chain!`]で定義された[`Error`]トレイトを実装した型に自動変換してくれます。

下記のレシピでは`/proc/uptime`を開くのと最初に取得した数値をパースするのにかかった時間を教えてくれます。エラーがない場合、実行時間を返します。

この本の他のレシピでは[error-chain] ボイラープレートは隠されていて、⤢ボタンで詳細をみた時に表示されます。

```rust
#[macro_use]
extern crate error_chain;

use std::fs::File;
use std::io::Read;

error_chain!{
    foreign_links {
        Io(std::io::Error);
        ParseInt(::std::num::ParseIntError);
    }
}

fn read_uptime() -> Result<u64> {
    let mut uptime = String::new();
    File::open("/proc/uptime")?.read_to_string(&mut uptime)?;

    Ok(uptime
        .split('.')
        .next()
        .ok_or("Cannot parse uptime data")?
        .parse()?)
}

fn main() {
    match read_uptime() {
        Ok(uptime) => println!("uptime: {} seconds", uptime),
        Err(err) => eprintln!("error: {}", err),
    };
}
```

[`error_chain!`]: https://docs.rs/error-chain/*/error_chain/macro.error_chain.html
[`Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
[`foreign_links`]: https://docs.rs/error-chain/*/error_chain/#foreign-links
[`std::io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html

[Rustでのエラーハンドリング]: https://doc.rust-lang.org/book/second-edition/ch09-00-error-handling.html
