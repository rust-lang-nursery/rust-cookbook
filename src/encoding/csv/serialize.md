## Serialize records to CSV

[![csv-badge]][csv] [![cat-encoding-badge]][cat-encoding]

This example shows how to serialize a Rust tuple. [`csv::writer`] supports automatic
serialization from Rust types into CSV records. [`write_record`] writes
a simple record containing string data only. Data with more complex values
such as numbers, floats, and options use [`serialize`]. Since CSV
writer uses internal buffer, always explicitly [`flush`] when done.

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
