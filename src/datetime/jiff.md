# Date and Time with jiff

[`jiff`] is a newer date and time crate. Its time zone support is built in
rather than an add-on, and [`Span`] represents calendar durations. These
recipes mirror some of the `chrono` recipes in this chapter; see
[Choosing between chrono, time and jiff] for how the crates compare.

{{#include jiff/now.md}}

{{#include jiff/parse_format.md}}

{{#include jiff/span.md}}

{{#include jiff/timestamp.md}}

[Choosing between chrono, time and jiff]: duration.html#choosing-between-chrono-time-and-jiff
[`jiff`]: https://docs.rs/jiff/

{{#include ../links.md}}
