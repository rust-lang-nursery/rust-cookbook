# Date and Time with time

[`time`] is a smaller date and time crate. It has no time zone database, only
fixed UTC offsets. Formats are written as compile-time checked format
descriptions. These recipes mirror some of the `chrono` and `jiff` recipes in
this chapter; see [Choosing between chrono, time and jiff] for how they compare.

{{#include time/now.md}}

{{#include time/parse_format.md}}

{{#include time/duration.md}}

{{#include time/timestamp.md}}

[Choosing between chrono, time and jiff]: jiff.html#choosing-between-chrono-time-and-jiff
[`time`]: https://docs.rs/time/

{{#include ../links.md}}
