# Date and Time with jiff

[`jiff`] is a newer date and time crate. Its time zone support is built in
rather than an add-on, and [`Span`] represents calendar durations. These
recipes mirror some of the `chrono` recipes in this chapter.

{{#include jiff/now.md}}

{{#include jiff/parse_format.md}}

{{#include jiff/span.md}}

{{#include jiff/timestamp.md}}

## Choosing between chrono, time and jiff

- [`chrono`] is the most widely used and has the largest ecosystem of
  integrations (for example `serde`, `diesel` and `sqlx`).
- [`time`] is a smaller, more conservative API with a focus on correctness.
- [`jiff`] bundles time zone handling, makes DST-aware arithmetic the default
  and has friendlier parsing and formatting. Prefer it for new code that works
  with time zones, and use `chrono` when a dependency already requires it.

[`chrono`]: https://docs.rs/chrono/
[`jiff`]: https://docs.rs/jiff/
[`Span`]: https://docs.rs/jiff/*/jiff/struct.Span.html
[`time`]: https://docs.rs/time/

{{#include ../links.md}}
