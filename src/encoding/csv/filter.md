## Filter CSV records matching a predicate

[![csv-badge]][csv] [![cat-encoding-badge]][cat-encoding]

Returns _only_ the rows from `data` with a field that matches `query`.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate csv;

use std::io;
#
# error_chain!{
#     foreign_links {
#         Io(std::io::Error);
#         CsvError(csv::Error);
#     }
# }

fn run() -> Result<()> {
    let query = "CA";
    let data = "\
City,State,Population,Latitude,Longitude
Kenai,AK,7610,60.5544444,-151.2583333
Oakman,AL,,33.7133333,-87.3886111
Sandfort,AL,,32.3380556,-85.2233333
West Hollywood,CA,37031,34.0900000,-118.3608333";

    let mut rdr = csv::ReaderBuilder::new().from_reader(data.as_bytes());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(rdr.headers()?)?;

    for result in rdr.records() {
        let record = result?;
        if record.iter().any(|field| field == query) {
            wtr.write_record(&record)?;
        }
    }

    wtr.flush()?;
    Ok(())
}
#
# quick_main!(run);
```

_Disclaimer: this example has been adapted from [the csv crate tutorial](https://docs.rs/csv/*/csv/tutorial/index.html#filter-by-search)_.
