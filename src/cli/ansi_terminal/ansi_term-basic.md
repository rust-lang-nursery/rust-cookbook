## ANSIターミナル

[![ansi_term-badge]][ansi_term] [![cat-command-line-badge]][cat-command-line]

このプログラムは、ANSIターミナルで[`ansi_term` crate]を使用し、青の太字テキストや黄色の下線付きテキストなど、色や書式を制御する方法を示しています。

**Note:** イギリス英語ではColorの代わりにColourを使います。困惑しないように。

### 色付き文字を出力する

```rust
extern crate ansi_term;

use ansi_term::Colour;

fn main() {
    println!("This is {} in color, {} in color and {} in color",
             Colour::Red.paint("red"),
             Colour::Blue.paint("blue"),
             Colour::Green.paint("green"));
}
```

### ターミナルで太文字

単純な前景色の変更よりも複雑なものについては、`Style`構造体をつくる必要があります。[`Style::new()`]は構造体を生成します。プロパティはメソッドチェーンで記述できます。

```rust
extern crate ansi_term;

use ansi_term::Style;

fn main() {
    println!("{} and this is not",
             Style::new().bold().paint("This is Bold"));
}
```
### ターミナルで色付き太文字

`Colour`は`Style`に似た関数をたくさん持っています。メソッドチェーンもできます。

```rust
extern crate ansi_term;

use ansi_term::Colour;
use ansi_term::Style;

fn main(){
    println!("{}, {} and {}",
             Colour::Yellow.paint("This is colored"),
             Style::new().bold().paint("this is bold"),
             Colour::Yellow.bold().paint("this is bold and colored"));
}
```

[documentation]: https://docs.rs/ansi_term/
[`ansi_term` crate]: https://crates.io/crates/ansi_term
[`ANSIString`]: https://docs.rs/ansi_term/*/ansi_term/type.ANSIString.html
[`Style`]: https://docs.rs/ansi_term/*/ansi_term/struct.Style.html
[`Style::new()`]: https://docs.rs/ansi_term/0.11.0/ansi_term/struct.Style.html#method.new
