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

Recipes are designed to give you instant access to working code, along
with a full explanation of what it is doing, and to guide you to
further information.

All recipes in the cookbook are full, self contained programs, so
that they may be copied directly into your own projects for
experimentation. To do so follow the instructions below.

Consider this example for "generate random numbers within a range":

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

```rust
extern crate rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Random f64: {}", rng.gen::<f64>());
}
```

To work with it locally we can run the following commands to create
a new cargo project, and change to that directory:


```sh
cargo new my-example --bin
cd my-example
```

Now, we also need to add the necessary crates to [Cargo.toml], as
indicated by the crate badges, in this case just "rand". To do so,
we'll use the `cargo add` command, which is provided by the
[`cargo-edit`] crate, which we need to install first:

```sh
cargo install cargo-edit
cargo add rand
```

Now you can replace `src/main.rs` with the full contents of the
example and run it:

```sh
cargo run
```

The crate badges that accompany the examples link to the crates' full
documentation on [docs.rs], and is often the next documentation you
should read after deciding which crate suites your purpose.

## エラーハンドリングについて

Error handling in Rust is robust when done correctly, but in today's
Rust it requires a fair bit of boilerplate. Because of this one often
sees Rust examples filled with `unwrap` calls instead of proper error
handling.

Since these recipes are intended to be reused as-is and encourage best
practices, they set up error handling correctly when there are
`Result` types involved.

The basic pattern we use is to have a `fn run() -> Result` that acts
like the "real" main function. We use the [error-chain] crate to make
`?` work within `run`.

The structure generally looks like:

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

For the sake of readability error handling boilerplate is hidden by
default like below.  In order to read full contents click on the
"expand" (<i class="fa fa-expand"></i>) button located in the top
right corner of the snippet.

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

For more background on error handling in Rust, read [this page of the
Rust book][error-docs] and [this blog post][error-blog].

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
