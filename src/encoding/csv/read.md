## Read CSV records

[![csv-badge]][csv] [![cat-encoding-badge]][cat-encoding]

Reads standard CSV records into [`csv::StringRecord`] — a weakly typed
data representation which expects valid UTF-8 rows. Alternatively,
[`csv::ByteRecord`] makes no assumptions about UTF-8.

```rust,edition2018
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

Serde deserializes data into strongly type structures. See the
[`csv::Reader::deserialize`] method.

```rust,edition2018
use serde::Deserialize;
#[derive(Deserialize)]
struct Record {
    year: u16,
    make: String,
    model: String,
    description: String,
}

fn main() -> Result<(), csv::Error> {
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
```

[`csv::ByteRecord`]: https://docs.rs/csv/*/csv/struct.ByteRecord.html
[`csv::Reader::deserialize`]: https://docs.rs/csv/*/csv/struct.Reader.html#method.deserialize
[`csv::StringRecord`]: https://docs.rs/csv/*/csv/struct.StringRecord.html
