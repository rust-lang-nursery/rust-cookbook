# About "Cookin' with Rust"

## テーブルコンテンツ

- [対象読者](#a対象読者)
- [どのようにこの本を読むか](#aどのようにこの本を読むか)
- [どのようにレシピを使うか](#aどのようにレシピを使うか)
- [エラーハンドリングについて](#aエラーハンドリングについて)
- [crateの表現について](#crateの表現について)

## 対象読者

この本ではRustの初心者を対象としており、Rustのクレートエコシステムの機能の概要をすぐに把握できます。また、一般的なタスクを簡単に成し遂げる方法を探すべき熟練Rustプログラマーも対象です。

## この本の読み方

クックブックの[索引]には"basics", "encoding", "concurrency"など、いくつかのセクションにまとめられたレシピのリストが載っています。セクションは大まかに順番に並べられています。後半のセクションは高度で時には前のセクションに基づいています。

索引にはそれぞれのセクションのレシピリストがあります。レシピは"範囲を指定して乱数を生成する”等、簡単なタスクを達成するものです。また、それぞれのレシピでは使われている _クレート_ が[![rand-badge]][rand]のようにタグづけされています。また、[![cat-science-badge]][cat-science]のように[crates.io]でどのカテゴリに属しているのかが載っています。

Rustプログラマー達は気楽に最初から最後のセクションまで読むべきです。そうすることでクレートエコシステムの概念をよく理解できるはずです。索引かサイドバーのセクションのヘッダーをクリックすることでそのセクションに移動することができます。

目的のレシピをクックブックから探すことは難しいです。レシピを探す一番簡単な方法は索引に目を通し、興味のあるクレートやカテゴリを見つけ、レシピの名前をクリックして見ることです。これは将来改善されるでしょう。

## レシピの使い方

レシピは、稼働中のコードにすぐに適用できるように設計されており、実行中のコードの詳細な説明とともに、さらに詳しい情報を提供します。

クックブックのすべてのレシピは完全な自己完結型プログラムなので、
あなたは実験的に自身のプロジェクトに直接コピーすることかもしれません。
その場合、以下の手順に従ってください。

"範囲を指定して乱数を作る"を例にします。:
[![rand-badge]][rand] [![cat-science-badge]][cat-science]

```rust
extern crate rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Random f64: {}", rng.gen::<f64>());
}
```

これをローカルで動かすには次のコマンドで新しいCargoプロジェクトを作り、そのディレクトリに移動しましょう。:

```sh
cargo new my-example --bin
cd my-example
```

次にクレートバッジに示されたクレートを[Cargo.toml]に追加する必要があります。この例では"rand"のみです。これをするのに、[`cargo-edit`]クレートが提供している`cargo add`コマンドを使います。これを最初にインストールする必要があります。:

```sh
cargo install cargo-edit
cargo add rand
```

これで、`src/main.rs`を例のコードに置き換えて動かすことができます。:

```sh
cargo run
```

クレートバッジは[docs.rs]にあるドキュメントへのリンクになっています。また、どのクレートを目的に使用するかを決定した後に、読んでください。

## エラーハンドリングについて

Rustでのエラーハンドリングは適切に扱えば堅牢ですが、今日のRustは少し定型文が必要です。なぜならRust examplesは本来のエラーハンドリングの代わりに`unwrap`が多用されています。

これらのレシピはそのまま再利用することを目的としたベストプラクティスなため、`Result`型がある時は適切なエラーハンドリングが設定されています。

基本的なパターンは"本当"のmain関数のように`fn run() -> Result`を使用することです。[error-chain]を使用して`?`は`run`内で動作します。

大抵の構造は以下の通り:

```rust
#[macro_use]
extern crate error_chain;

use std::net::IpAddr;
use std::str;

error_chain! {
    foreign_links {
        Utf8(std::str::Utf8Error);
        AddrParse(std::net::AddrParseError);
    }
}

fn run() -> Result<()> {
    let bytes = b"2001:db8::1";

    // Bytes to string.
    let s = str::from_utf8(bytes)?;

    // String to IP address.
    let addr: IpAddr = s.parse()?;

    println!("{:?}", addr);
    Ok(())
}

quick_main!(run);
```

This is using the `error_chain!` macro to define a custom `Error` and
`Result` type, along with automatic conversions from two standard
library error types. The automatic conversions make the `?` operator
work. The `quick_main!` macro generates the actual `main` function and
prints out the error if one occurred.

これはカスタム`Error`, `Result`型を定義するために`error_chain!`マクロを使って、2つの標準ライブラリーエラー型から自動変換します。自動変換は`?`演算子を作成します。`quick_main!`マクロは`main`関数を作り、エラーが起きたら出力します。

可読性のためにエラーハンドリングの定型文は下記のように隠されています。右上にある"expand" (<i class="fa fa-expand"></i>)ボタンをクリックすると全コードを読むことができます。

```rust
# #[macro_use]
# extern crate error_chain;
extern crate url;

use url::{Url, Position};
#
# error_chain! {
#     foreign_links {
#         UrlParse(url::ParseError);
#     }
# }

fn run() -> Result<()> {
    let parsed = Url::parse("https://httpbin.org/cookies/set?k2=v2&k1=v1")?;
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("cleaned: {}", cleaned);
    Ok(())
}
#
# quick_main!(run);
```

より詳細なRustでのエラーハンドリングについては[Rust bookのこのページ][error-docs] と[このブログ記事][error-blog]を読んでください。

## crateの表現について

This cookbook is intended eventually to provide expansive coverage of
the Rust crate ecosystem, but today is limited in scope while we get
it bootstrapped and work on the presentation. Hopefully, starting
from a small scope and slowly expanding will help the cookbook become
a high-quality resource sooner, and allow it to maintain consistent
quality levels as it grows.

At present the cookbook is focused on the standard library, and on
"core", or "foundational", crates—those crates that make up the most
common programming tasks, and that the rest of the ecosystem builds
off of.

The cookbook is closely tied to the [Rust Libz Blitz], a project to
identify, and improve the quality of such crates, and so it largely
defers crate selection to that project. Any crates that have already
been evaluated as part of that process are in scope for the cookbook,
as are crates that are pending evaluation.

{{#include links.md}}

[index]: intro.html
[error-docs]: https://doc.rust-lang.org/book/error-handling.html
[error-blog]: https://brson.github.io/2016/11/30/starting-with-error-chain
[error-chain]: https://docs.rs/error-chain/
[Rust Libz Blitz]: https://internals.rust-lang.org/t/rust-libz-blitz/5184
[crates.io]: https://crates.io
[docs.rs]: https://docs.rs
[Cargo.toml]: http://doc.crates.io/manifest.html
[`cargo-edit`]: https://github.com/killercup/cargo-edit
