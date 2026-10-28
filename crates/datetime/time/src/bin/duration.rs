use time::macros::datetime;
use time::{Duration, OffsetDateTime};

fn main() {
    let start = datetime!(2024-03-09 12:00 -5);

    // A `Duration` is an exact length of time, so a "day" is always 24 hours.
    // There is no time zone database, so DST changes are not applied.
    let next_day = start.checked_add(Duration::days(1));
    println!("{start} + 1 day = {next_day:?}");

    let later = start
        .checked_add(Duration::weeks(2))
        .and_then(|t| t.checked_add(Duration::hours(27)));
    println!("Two weeks and 27 hours later: {later:?}");

    // `checked_add` returns `None` on overflow instead of panicking.
    let overflow = start.checked_add(Duration::MAX);
    println!("Overflow is None: {}", overflow.is_none());

    let end: OffsetDateTime = datetime!(2024-03-12 09:30 -4);
    let diff = end - start;
    println!(
        "Until {end}: {}h {}m",
        diff.whole_hours(),
        diff.whole_minutes() % 60
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_arithmetic() {
        let start = datetime!(2024-03-09 12:00 -5);
        assert_eq!(
            start.checked_add(Duration::days(1)),
            Some(datetime!(2024-03-10 12:00 -5))
        );
        assert!(start.checked_add(Duration::MAX).is_none());

        let diff = datetime!(2024-03-12 09:30 -4) - start;
        assert_eq!((diff.whole_hours(), diff.whole_minutes() % 60), (68, 30));
    }
}
