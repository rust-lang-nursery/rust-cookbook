use time::{OffsetDateTime, UtcOffset};

fn main() -> Result<(), time::error::ComponentRange> {
    let now = OffsetDateTime::now_utc();
    println!("UTC time:   {now}");

    // `time` has no time zone database, only fixed UTC offsets.
    let tokyo = now.to_offset(UtcOffset::from_hms(9, 0, 0)?);
    println!("UTC+09:00:  {tokyo}");

    println!(
        "Year {}, month {}, day {}, weekday {}",
        now.year(),
        now.month(),
        now.day(),
        now.weekday()
    );
    println!(
        "Time {:02}:{:02}:{:02}",
        now.hour(),
        now.minute(),
        now.second()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_offset_without_changing_the_instant() -> Result<(), time::error::ComponentRange> {
        let now = OffsetDateTime::now_utc();
        let tokyo = now.to_offset(UtcOffset::from_hms(9, 0, 0)?);
        assert_eq!(tokyo, now);
        assert_eq!(tokyo.offset().whole_hours(), 9);
        Ok(())
    }
}
