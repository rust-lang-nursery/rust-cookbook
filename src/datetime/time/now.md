## Examine the current time with a UTC offset

[![time-badge]][time] [![cat-date-and-time-badge]][cat-date-and-time]

Use `time` when fixed UTC offsets are enough and you want a small API; see
[Choosing between chrono, time and jiff]. Gets the current [`OffsetDateTime`] in UTC and converts it to a fixed
offset with [`OffsetDateTime::to_offset`]. `time` has no time zone database, so
named zones such as `Asia/Tokyo` are not available and daylight saving time is
never applied. If you need either, use `jiff` or `chrono` with `chrono-tz`.

```rust
{{#include ../../../crates/datetime/time/src/bin/now.rs::25 }}
```

[`OffsetDateTime`]: https://docs.rs/time/*/time/struct.OffsetDateTime.html
[`OffsetDateTime::to_offset`]: https://docs.rs/time/*/time/struct.OffsetDateTime.html#method.to_offset
