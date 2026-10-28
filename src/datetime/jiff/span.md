## Perform calendar arithmetic with Span

[![jiff-badge]][jiff] [![cat-date-and-time-badge]][cat-date-and-time]

A [`Span`] is a duration made of calendar units such as days and weeks as well
as clock units. [`Zoned::checked_add`] applies it with daylight saving time
taken into account, and returns an `Err` rather than panicking on overflow.
[`Zoned::until`] computes the difference between two datetimes.

```rust
{{#include ../../../crates/datetime/jiff/src/bin/span.rs::37 }}
```

[`Span`]: https://docs.rs/jiff/*/jiff/struct.Span.html
[`Zoned::checked_add`]: https://docs.rs/jiff/*/jiff/struct.Zoned.html#method.checked_add
[`Zoned::until`]: https://docs.rs/jiff/*/jiff/struct.Zoned.html#method.until
