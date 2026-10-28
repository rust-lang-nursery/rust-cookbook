## Convert a datetime to a UNIX timestamp and vice versa

[![time-badge]][time] [![cat-date-and-time-badge]][cat-date-and-time]

Builds an [`OffsetDateTime`] from a [`Date`] with [`PrimitiveDateTime::assume_utc`]
and reads its seconds since the UNIX epoch with
[`OffsetDateTime::unix_timestamp`]. [`OffsetDateTime::from_unix_timestamp`]
goes the other way.

```rust
{{#include ../../../crates/datetime/time/src/bin/timestamp.rs::16 }}
```

[`Date`]: https://docs.rs/time/*/time/struct.Date.html
[`OffsetDateTime`]: https://docs.rs/time/*/time/struct.OffsetDateTime.html
[`OffsetDateTime::from_unix_timestamp`]: https://docs.rs/time/*/time/struct.OffsetDateTime.html#method.from_unix_timestamp
[`OffsetDateTime::unix_timestamp`]: https://docs.rs/time/*/time/struct.OffsetDateTime.html#method.unix_timestamp
[`PrimitiveDateTime::assume_utc`]: https://docs.rs/time/*/time/struct.PrimitiveDateTime.html#method.assume_utc
