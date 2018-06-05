## Examine the date and time

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Gets the current UTC [`DateTime`] and its hour/minute/second via [`Timelike`]
and its year/month/day/weekday via [`Datelike`].

```rust
extern crate chrono;
use chrono::{Datelike, Timelike, Utc};

fn main() {
    let now = Utc::now();

    let (is_pm, hour) = now.hour12();
    println!(
        "The current UTC time is {:02}:{:02}:{:02} {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );
    println!(
        "And there have been {} seconds since midnight",
        now.num_seconds_from_midnight()
    );

    let (is_common_era, year) = now.year_ce();
    println!(
        "The current UTC date is {}-{:02}-{:02} {:?} ({})",
        year,
        now.month(),
        now.day(),
        now.weekday(),
        if is_common_era { "CE" } else { "BCE" }
    );
    println!(
        "And the Common Era began {} days ago",
        now.num_days_from_ce()
    );
}
```

[`Datelike`]: https://docs.rs/chrono/*/chrono/trait.Datelike.html
[`DateTime`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html
[`Timelike`]: https://docs.rs/chrono/*/chrono/trait.Timelike.html
