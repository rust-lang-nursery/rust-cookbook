use jiff::{Timestamp, civil};

fn main() -> Result<(), jiff::Error> {
    let zoned = civil::date(2017, 11, 12).at(17, 33, 44, 0).in_tz("UTC")?;
    let timestamp = zoned.timestamp();
    println!(
        "Number of seconds between 1970-01-01 00:00:00 and {} is {}.",
        zoned.datetime(),
        timestamp.as_second()
    );

    let after_a_billion = Timestamp::from_second(1_000_000_000)?;
    println!("Date after a billion seconds since 1970-01-01 00:00:00 was {after_a_billion}.");

    println!(
        "The same instant in Los Angeles: {}",
        after_a_billion.in_tz("America/Los_Angeles")?
    );
    Ok(())
}
