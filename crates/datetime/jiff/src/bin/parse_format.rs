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
