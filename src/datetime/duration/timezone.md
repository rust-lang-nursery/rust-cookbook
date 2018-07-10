## Convert a local time to another timezone

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Gets the local time and displays it using [`offset::Local::now`] and then converts it to the UTC standard using the [`DateTime::from_utc`] struct method. A time is then converted using the [`offset::FixedOffset`] struct and the UTC time is then converted to UTC+8 and UTC-2.

```rust
extern crate chrono;

use chrono::{DateTime, FixedOffset, Local, Utc};

fn main() {
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    let china_timezone = FixedOffset::east(8 * 3600);
    let rio_timezone = FixedOffset::west(2 * 3600);
    println!("Local time now is {}", local_time);
    println!("UTC time now is {}", utc_time);
    println!(
        "Time in Hong Kong now is {}",
        utc_time.with_timezone(&china_timezone)
    );
    println!("Time in Rio de Janeiro now is {}", utc_time.with_timezone(&rio_timezone));
}
```

[`DateTime::from_utc`]:https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.from_utc
[`offset::FixedOffset`]: https://docs.rs/chrono/*/chrono/offset/struct.FixedOffset.html
[`offset::Local::now`]: https://docs.rs/chrono/*/chrono/offset/struct.Local.html#method.now
