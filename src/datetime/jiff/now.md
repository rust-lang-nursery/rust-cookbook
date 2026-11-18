## Examine the current time with a time zone

[![jiff-badge]][jiff] [![cat-date-and-time-badge]][cat-date-and-time]

Use `jiff` when you need named IANA time zones without an extra crate; see
[Choosing between chrono, time and jiff]. Gets the current [`ZonedDateTime`] datetime in the system time zone, converts it
to another zone such as `Asia/Tokyo` with [`ZonedDateTime::in_tz`], and reads its
parts. A [`ZonedDateTime`] carries its time zone, so the UTC offset is always correct
for that instant. `time` only has fixed offsets, so it cannot do this lookup.

```rust
{{#include ../../../crates/datetime/jiff/src/bin/now.rs::28 }}
```

[`ZonedDateTime`]: https://docs.rs/jiff/*/jiff/struct.ZonedDateTime.html
[`ZonedDateTime::in_tz`]: https://docs.rs/jiff/*/jiff/struct.ZonedDateTime.html#method.in_tz
