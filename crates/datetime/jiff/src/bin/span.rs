use jiff::{Span, SpanRound, Unit, Zoned, civil};

fn main() -> Result<(), jiff::Error> {
    let start = civil::date(2024, 3, 9)
        .at(12, 0, 0, 0)
        .in_tz("America/New_York")?;

    // Calendar-aware arithmetic: clocks spring forward on March 10, so that day
    // is only 23 hours long. Adding one day still lands on 12:00, while adding
    // 24 hours lands on 13:00.
    let next_day = start.checked_add(Span::new().days(1))?;
    println!("{start} + 1 day   = {next_day}");

    let in_24_hours = start.checked_add(Span::new().hours(24))?;
    println!("{start} + 24 hours = {in_24_hours}");

    // Mixed units, and checked_add returns `Err` on overflow instead of panicking.
    let later = start.checked_add(Span::new().weeks(2).days(1).hours(3))?;
    println!("Two weeks, a day and 3 hours later: {later}");

    let overflow = start.checked_add(Span::new().years(10_000));
    println!("Overflow is an error: {}", overflow.is_err());

    // The difference between two datetimes, balanced up to hours.
    let end: Zoned = "2024-03-12T09:30:00-04:00[America/New_York]".parse()?;
    let diff = start.until((Unit::Hour, &end))?;
    println!("Until {end}: {diff:#}");

    let rounded = start.until(&end)?.round(
        SpanRound::new()
            .largest(Unit::Day)
            .smallest(Unit::Hour)
            .relative(&start),
    )?;
    println!("Rounded: {rounded:#}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spans_respect_dst() -> Result<(), jiff::Error> {
        let start = civil::date(2024, 3, 9)
            .at(12, 0, 0, 0)
            .in_tz("America/New_York")?;
        let next_day = start.checked_add(Span::new().days(1))?;
        assert_eq!(next_day.hour(), 12);
        let in_24_hours = start.checked_add(Span::new().hours(24))?;
        assert_eq!(in_24_hours.hour(), 13);
        assert!(start.checked_add(Span::new().years(10_000)).is_err());

        let end: Zoned = "2024-03-12T09:30:00-04:00[America/New_York]".parse()?;
        let diff = start.until((Unit::Hour, &end))?;
        assert_eq!((diff.get_hours(), diff.get_minutes()), (68, 30));
        Ok(())
    }
}
