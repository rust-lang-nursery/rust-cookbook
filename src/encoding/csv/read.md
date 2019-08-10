## CSVレコードを読み込む

[![csv-badge]][csv] [![cat-encoding-badge]][cat-encoding]

標準なCSVレコードを[`csv::StringRecord`]に読み込みます。- 弱く型付けされた有効なUTF-8行のデータ。または、[`csv::ByteRecord`]を使えばUTF-8について考える必要がなくなります。

```rust
extern crate csv;
use csv::Error;

fn main() -> Result<(), Error> {
    let csv = "year,make,model,description
		1948,Porsche,356,Luxury sports car
		1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.records() {
        let record = record?;
        println!(
            "In {}, {} built the {} model. It is a {}.",
            &record[0],
            &record[1],
            &record[2],
            &record[3]
        );
    }

    Ok(())
}
```
Serdeデシリアライズで強く型付けされた構造体にする。[`csv::Reader::deserialize`]メソッドをみてください。

```rust
extern crate csv;
# #[macro_use]
# extern crate error_chain;
#[macro_use]
extern crate serde_derive;

# error_chain! {
#     foreign_links {
#         Reader(csv::Error);
#     }
# }
#
#[derive(Deserialize)]
struct Record {
    year: u16,
    make: String,
    model: String,
    description: String,
}

fn run() -> Result<()> {
    let csv = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    for record in reader.deserialize() {
        let record: Record = record?;
        println!(
            "In {}, {} built the {} model. It is a {}.",
            record.year,
            record.make,
            record.model,
            record.description
        );
    }

    Ok(())
}
#
# quick_main!(run);
```

[`csv::ByteRecord`]: https://docs.rs/csv/*/csv/struct.ByteRecord.html
[`csv::Reader::deserialize`]: https://docs.rs/csv/*/csv/struct.Reader.html#method.deserialize
[`csv::StringRecord`]: https://docs.rs/csv/*/csv/struct.StringRecord.html
