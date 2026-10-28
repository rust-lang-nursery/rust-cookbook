## Perform checked date and time calculations with Duration

[![time-badge]][time] [![cat-date-and-time-badge]][cat-date-and-time]

Use `time` when exact elapsed time is what you want; see [Choosing between chrono, time and jiff]. Adds a
[`Duration`] with [`OffsetDateTime::checked_add`], which returns `None` on
overflow. A [`Duration`] is an exact length of time, so a day is always 24
hours and daylight saving time is not applied: this is the opposite of the
`jiff` `Span` recipe, where adding a day can be 23 or 25 hours. Subtracting two
datetimes gives the [`Duration`] between them.

```rust
{{#include ../../../crates/datetime/time/src/bin/duration.rs::28 }}
```

[`Duration`]: https://docs.rs/time/*/time/struct.SignedDuration.html
[`OffsetDateTime::checked_add`]: https://docs.rs/time/*/time/struct.OffsetDateTime.html#method.checked_add
