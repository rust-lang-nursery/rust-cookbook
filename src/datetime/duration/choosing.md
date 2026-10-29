## Choosing between chrono, time and jiff
<!-- not-a-recipe -->

- [`chrono`] and [`time`] are both very widely used, and both are supported by
  crates such as `serde`, `diesel` and `sqlx`. `chrono` has a larger API, but
  needs the separate [`chrono-tz`] crate for IANA time zones. `time` is smaller
  and only knows fixed UTC offsets, with no time zone database.
- [`jiff`] has IANA time zone support built in, using the system's time zone
  database, or a bundled copy on platforms without one. Its [`Span`] and
  [`Zoned`] types make DST-aware arithmetic the default. It has fewer
  third-party integrations so far.
- All three are pre-1.0, so their APIs may still change.

If your code works with time zones and has no existing `chrono` or `time`
dependency, `jiff` is worth considering. Otherwise stay with whatever your
dependencies already use.

[`chrono`]: https://docs.rs/chrono/
[`chrono-tz`]: https://docs.rs/chrono-tz/
[`jiff`]: https://docs.rs/jiff/
[`Span`]: https://docs.rs/jiff/*/jiff/struct.Span.html
[`time`]: https://docs.rs/time/
[`Zoned`]: https://docs.rs/jiff/*/jiff/struct.Zoned.html
