## Convert a datetime to a UNIX timestamp and vice versa

[![jiff-badge]][jiff] [![cat-date-and-time-badge]][cat-date-and-time]

Converts a civil datetime to a [`Timestamp`] and reads its seconds since the
UNIX epoch with [`Timestamp::as_second`]. [`Timestamp::from_second`] goes the
other way, and [`Timestamp::in_tz`] shows the instant in any time zone.

```rust
{{#include ../../../crates/datetime/jiff/src/bin/timestamp.rs::20 }}
```

[`Timestamp`]: https://docs.rs/jiff/*/jiff/struct.Timestamp.html
[`Timestamp::as_second`]: https://docs.rs/jiff/*/jiff/struct.Timestamp.html#method.as_second
[`Timestamp::from_second`]: https://docs.rs/jiff/*/jiff/struct.Timestamp.html#method.from_second
[`Timestamp::in_tz`]: https://docs.rs/jiff/*/jiff/struct.Timestamp.html#method.in_tz
