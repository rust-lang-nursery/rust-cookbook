use time::format_description::well_known::Rfc3339;
use time::macros::format_description;
use time::{Date, OffsetDateTime};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed = OffsetDateTime::parse("2024-07-11T01:14:00-04:00", &Rfc3339)?;
    println!("Parsed:    {parsed}");

    let format = format_description!(
        "[weekday], [month repr:long] [day padding:none], [year] at [hour repr:12 padding:none]:[minute] [period] [offset_hour sign:mandatory]:[offset_minute]"
    );
    println!("Formatted: {}", parsed.format(&format)?);

    // Parsing with a custom format description.
    let date = Date::parse("11/07/2024", format_description!("[day]/[month]/[year]"))?;
    println!("Custom:    {date}");

    // Invalid input is an `Err`, not a panic.
    let bad = OffsetDateTime::parse("not a date", &Rfc3339);
    println!("Invalid:   {}", bad.is_err());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_formats() -> Result<(), Box<dyn std::error::Error>> {
        let parsed = OffsetDateTime::parse("2024-07-11T01:14:00-04:00", &Rfc3339)?;
        let format = format_description!(
            "[weekday], [month repr:long] [day padding:none], [year] at [hour repr:12 padding:none]:[minute] [period] [offset_hour sign:mandatory]:[offset_minute]"
        );
        assert_eq!(
            parsed.format(&format)?,
            "Thursday, July 11, 2024 at 1:14 AM -04:00"
        );
        let date = Date::parse("11/07/2024", format_description!("[day]/[month]/[year]"))?;
        assert_eq!(date, time::macros::date!(2024 - 07 - 11));
        assert!(OffsetDateTime::parse("not a date", &Rfc3339).is_err());
        Ok(())
    }
}
