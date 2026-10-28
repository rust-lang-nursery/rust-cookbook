# Time Zones

[`jiff`] is a newer date and time crate. Its time zone support is built in
rather than an add-on, and [`Span`] represents calendar durations. These
recipes show what `jiff` does differently from `chrono` and `time`; see
[Choosing between chrono, time and jiff] for how the crates compare.

{{#include jiff/now.md}}

{{#include jiff/parse_format.md}}

{{#include jiff/span.md}}

[Choosing between chrono, time and jiff]: duration.html#choosing-between-chrono-time-and-jiff
[`jiff`]: https://docs.rs/jiff/

{{#include ../links.md}}
