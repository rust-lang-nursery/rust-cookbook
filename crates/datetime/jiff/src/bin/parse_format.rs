use jiff::Zoned;
use jiff::civil::Date;
use jiff::fmt::strtime;

fn main() -> Result<(), jiff::Error> {
    // RFC 9557 strings carry an IANA time zone name alongside the offset.
    let zoned: Zoned = "2024-07-11T01:14:00-04:00[America/New_York]".parse()?;
    println!("Parsed:    {zoned}");
    println!(
        "Formatted: {}",
        zoned.strftime("%A, %B %-d, %Y at %-I:%M %p %Z")
    );

    // Parsing with a custom strftime-style pattern.
    let date: Date = strtime::parse("%d/%m/%Y", "11/07/2024")?.to_date()?;
    println!("Custom:    {date}");

    // Invalid input is an `Err`, not a panic.
    let bad = "not a date".parse::<Zoned>();
    println!("Invalid:   {}", bad.is_err());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_formats() -> Result<(), jiff::Error> {
        let zoned: Zoned = "2024-07-11T01:14:00-04:00[America/New_York]".parse()?;
        assert_eq!(
            zoned.strftime("%A, %B %-d, %Y at %-I:%M %p %Z").to_string(),
            "Thursday, July 11, 2024 at 1:14 AM EDT"
        );
        let date: Date = strtime::parse("%d/%m/%Y", "11/07/2024")?.to_date()?;
        assert_eq!(date, jiff::civil::date(2024, 7, 11));
        assert!("not a date".parse::<Zoned>().is_err());
        Ok(())
    }
}
