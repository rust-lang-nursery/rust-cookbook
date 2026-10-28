## Parse and format a datetime with format descriptions

[![time-badge]][time] [![cat-date-and-time-badge]][cat-date-and-time]

Use `time` when you want format strings checked at compile time; see
[Choosing between chrono, time and jiff]. Parses an RFC 3339 string with [`OffsetDateTime::parse`] and [`Rfc3339`].
Custom patterns are written with the [`format_description!`] macro, so a typo
in `"[hour repr:12]"` is a build error rather than a runtime one, and used with
[`OffsetDateTime::format`] and [`Date::parse`]. Invalid input returns an
`Err`.

```rust
{{#include ../../../crates/datetime/time/src/bin/parse_format.rs::22 }}
```

[`Date::parse`]: https://docs.rs/time/*/time/struct.Date.html#method.parse
[`format_description!`]: https://docs.rs/time/*/time/macros/macro.format_description.html
[`OffsetDateTime::format`]: https://docs.rs/time/*/time/struct.OffsetDateTime.html#method.format
[`OffsetDateTime::parse`]: https://docs.rs/time/*/time/struct.OffsetDateTime.html#method.parse
[`Rfc3339`]: https://docs.rs/time/*/time/format_description/well_known/struct.Rfc3339.html
