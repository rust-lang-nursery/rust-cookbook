## レコードをCSVにシリアライズする

[![csv-badge]][csv] [![cat-encoding-badge]][cat-encoding]

この例ではRustのタプルをシリアライズする方法をお見せします。[`csv::writer`]はRustの型からCSVレコードへの自動シリアライズをサポートしています。[`write_record`]は文字列のみのシンプルなレコードを書き込みます。数値、浮動小数、optionのような複雑な値を持ったデータは[`serialize`]を使います。CSV writerは内部のバッファを使い、終了時に明示的に[`flush`]します。

```rust
# #[macro_use]
# extern crate error_chain;
extern crate csv;

use std::io;
#
# error_chain! {
#     foreign_links {
#         CSVError(csv::Error);
#         IOError(std::io::Error);
#    }
# }

fn run() -> Result<()> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(&["Name", "Place", "ID"])?;

    wtr.serialize(("Mark", "Sydney", 87))?;
    wtr.serialize(("Ashley", "Dublin", 32))?;
    wtr.serialize(("Akshat", "Delhi", 11))?;

    wtr.flush()?;
    Ok(())
}
#
# quick_main!(run);
```

[`csv::Writer`]: https://docs.rs/csv/*/csv/struct.Writer.html
[`flush`]: https://docs.rs/csv/*/csv/struct.Writer.html#method.flush
[`serialize`]: https://docs.rs/csv/*/csv/struct.Writer.html#method.serialize
[`write_record`]: https://docs.rs/csv/*/csv/struct.Writer.html#method.write_record
