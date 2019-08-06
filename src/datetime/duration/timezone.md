## ローカルタイムを他のタイムゾーンに変換する

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

[`offset::Local::now`]を使いローカルタイムを取得し表示して、[`DateTime::from_utc`]構造体メソッドを使ってUTCスタンダードに変換する。時間は[`offset::FixedOffset`]構造体に変換され、UTCタイムはUTC+8とUTC-2に変換される。

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
