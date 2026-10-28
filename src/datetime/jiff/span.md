## Perform calendar arithmetic with Span

[![jiff-badge]][jiff] [![cat-date-and-time-badge]][cat-date-and-time]

Use `jiff` when calendar arithmetic has to follow daylight saving time; see
[Choosing between chrono, time and jiff]. A [`Span`] is a duration made of calendar units such as days and weeks as
well as clock units. [`Zoned::checked_add`] applies it with the time zone taken
into account, so adding one day to 12:00 before a spring-forward change gives
12:00 the next day, while adding 24 hours gives 13:00. A fixed-offset type such
as `time`'s `OffsetDateTime` cannot tell these apart. `checked_add` returns an
`Err` rather than panicking on overflow, and [`Zoned::until`] computes the
difference between two datetimes.

```rust
{{#include ../../../crates/datetime/jiff/src/bin/span.rs::37 }}
```

[`Span`]: https://docs.rs/jiff/*/jiff/struct.Span.html
[`Zoned::checked_add`]: https://docs.rs/jiff/*/jiff/struct.Zoned.html#method.checked_add
[`Zoned::until`]: https://docs.rs/jiff/*/jiff/struct.Zoned.html#method.until
