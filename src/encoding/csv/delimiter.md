## Read CSV records with different delimiter

[![csv-badge]][csv] [![cat-encoding-badge]][cat-encoding]

Reads CSV records with a tab [`delimiter`].

```rust,edition2018
use csv::Error;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

use csv::ReaderBuilder;

fn main() -> Result<(), Error> {
    let data = "name\tplace\tid\n\
        Mark\tMelbourne\t46\n\
        Ashley\tZurich\t92";

    let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(data.as_bytes());
    for result in reader.deserialize::<Record>() {
        println!("{:?}", result?);
    }

    Ok(())
}
```