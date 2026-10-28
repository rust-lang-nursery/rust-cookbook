## Examine the current time with a time zone

[![jiff-badge]][jiff] [![cat-date-and-time-badge]][cat-date-and-time]

Gets the current [`Zoned`] datetime in the system time zone, converts it to
another IANA time zone with [`Zoned::in_tz`], and reads its parts. A [`Zoned`]
carries its time zone, so the UTC offset is always correct for that instant.

```rust
{{#include ../../../crates/datetime/jiff/src/bin/now.rs::28 }}
```

[`Zoned`]: https://docs.rs/jiff/*/jiff/struct.Zoned.html
[`Zoned::in_tz`]: https://docs.rs/jiff/*/jiff/struct.Zoned.html#method.in_tz
