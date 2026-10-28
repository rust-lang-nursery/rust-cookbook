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
