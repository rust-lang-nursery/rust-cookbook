## Perform checked date and time calculations with Duration

[![time-badge]][time] [![cat-date-and-time-badge]][cat-date-and-time]

Adds a [`Duration`] with [`OffsetDateTime::checked_add`], which returns `None`
on overflow. A [`Duration`] is an exact length of time, so a day is always 24
hours and daylight saving time is not applied. Subtracting two datetimes gives
the [`Duration`] between them.

```rust
{{#include ../../../crates/datetime/time/src/bin/duration.rs::28 }}
```

[`Duration`]: https://docs.rs/time/*/time/struct.Duration.html
[`OffsetDateTime::checked_add`]: https://docs.rs/time/*/time/struct.OffsetDateTime.html#method.checked_add
