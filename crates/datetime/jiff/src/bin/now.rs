use jiff::{Timestamp, Zoned};

fn main() -> Result<(), jiff::Error> {
    let now = Zoned::now();
    println!("Local time: {now}");

    let tokyo = now.in_tz("Asia/Tokyo")?;
    println!("In Tokyo:   {tokyo}");
    println!("UTC offset: {}", tokyo.offset());

    let timestamp: Timestamp = now.timestamp();
    println!("Timestamp:  {timestamp}");

    println!(
        "Year {}, month {}, day {}, weekday {:?}",
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
    fn converts_zone_without_changing_the_instant() -> Result<(), jiff::Error> {
        let now = Zoned::now();
        let tokyo = now.in_tz("Asia/Tokyo")?;
        assert_eq!(tokyo.timestamp(), now.timestamp());
        assert_eq!(tokyo.offset().seconds(), 9 * 3600);
        Ok(())
    }
}
