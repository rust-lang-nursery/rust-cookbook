## Examine the current time with a UTC offset

[![time-badge]][time] [![cat-date-and-time-badge]][cat-date-and-time]

Gets the current [`OffsetDateTime`] in UTC and converts it to a fixed offset
with [`OffsetDateTime::to_offset`]. `time` has no time zone database, so
named zones such as `Asia/Tokyo` are not available.

```rust
{{#include ../../../crates/datetime/time/src/bin/now.rs::25 }}
```

[`OffsetDateTime`]: https://docs.rs/time/*/time/struct.OffsetDateTime.html
[`OffsetDateTime::to_offset`]: https://docs.rs/time/*/time/struct.OffsetDateTime.html#method.to_offset
