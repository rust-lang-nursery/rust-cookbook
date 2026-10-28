use time::{Date, Month, OffsetDateTime};

fn main() -> Result<(), time::error::ComponentRange> {
    let date_time = Date::from_calendar_date(2017, Month::November, 12)?
        .with_hms(17, 33, 44)?
        .assume_utc();
    println!(
        "Number of seconds between 1970-01-01 00:00:00 and {} is {}.",
        date_time,
        date_time.unix_timestamp()
    );

    let after_a_billion = OffsetDateTime::from_unix_timestamp(1_000_000_000)?;
    println!("Date after a billion seconds since 1970-01-01 00:00:00 was {after_a_billion}.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_to_and_from_timestamps() -> Result<(), time::error::ComponentRange> {
        let date_time = Date::from_calendar_date(2017, Month::November, 12)?
            .with_hms(17, 33, 44)?
            .assume_utc();
        assert_eq!(date_time.unix_timestamp(), 1_510_508_024);
        assert_eq!(
            OffsetDateTime::from_unix_timestamp(1_000_000_000)?,
            time::macros::datetime!(2001-09-09 1:46:40 UTC)
        );
        Ok(())
    }
}
