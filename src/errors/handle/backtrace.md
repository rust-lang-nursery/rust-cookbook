## Obtain backtrace of complex error scenarios

[![anyhow-badge]][anyhow] [![cat-rust-patterns-badge]][cat-rust-patterns]

This recipe shows how to handle a complex error scenario and then
print a backtrace. It relies on [`anyhow::Context`] to extend errors by
appending new errors. The error stack can be unwound, thus providing
a better context to understand why an error was raised.

The below recipes attempts to deserialize the value `256` into a
`u8`. An error will bubble up from Serde then csv and finally up to the
user code.

```rust,edition2018
extern crate anyhow;
extern crate csv;
extern crate serde;
use anyhow::{Result, Context};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Rgb {
    red: u8,
    blue: u8,
    green: u8,
}

impl std::fmt::UpperHex for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

impl Rgb {
    fn from_reader(csv_data: &[u8]) -> Result<Rgb> {
        let color: Rgb = csv::Reader::from_reader(csv_data)
            .deserialize()
            .nth(0)
            .ok_or_else(|| anyhow::anyhow!("Cannot deserialize the first CSV record"))?
            .context("Cannot deserialize RGB color")?;

        Ok(color)
    }
}

fn run() -> Result<()> {
    let csv = "red,blue,green
102,256,204";

    let rgb = Rgb::from_reader(csv.as_bytes()).context("Cannot read CSV data")?;
    println!("{:?} to hexadecimal #{:X}", rgb, rgb);

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        eprintln!("Backtrace:");
        eprintln!("{:?}", error.backtrace());
    }
}
```

Backtrace error rendered:

```text
Error: Cannot read CSV data

Caused by:
    Cannot deserialize RGB color

Caused by:
    CSV deserialize error: record 1 (line: 2, byte: 15): field 1: number too large to fit in target type

Caused by:
    field 1: number too large to fit in target type
```

Run the recipe with `RUST_BACKTRACE=1` to display a detailed backtrace associated with this error.

[`anyhow`]: https://docs.rs/anyhow/latest/anyhow/
[`anyhow::Context`]: https://docs.rs/anyhow/latest/anyhow/trait.Context.html
