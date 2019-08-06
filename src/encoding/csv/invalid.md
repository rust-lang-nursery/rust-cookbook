## Serdeで無効なCSVデータをハンドル

[![csv-badge]][csv] [![serde-badge]][serde] [![cat-encoding-badge]][cat-encoding]

CSVファイルはしばしば無効なデータを含みます。これらの場合`csv`はカスタムデシリアライザーである[`csv::invalid_option`]を提供します。これは無効なデータを自動でNone値に変換してくれます。

```rust
extern crate csv;
use csv::Error;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

fn main() -> Result<(), Error> {
    let data = "name,place,id
mark,sydney,46.5
ashley,zurich,92
akshat,delhi,37
alisha,colombo,xyz";

    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
```

[`csv::invalid_option`]: https://docs.rs/csv/*/csv/fn.invalid_option.html
