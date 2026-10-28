# Date and Time with time

[`time`] is a smaller date and time crate. It has no time zone database, only
fixed UTC offsets. Formats are written as compile-time checked format
descriptions. These recipes show where `time` differs from `chrono` and `jiff`; see
[Choosing between chrono, time and jiff] for how they compare.

{{#include time/now.md}}

{{#include time/parse_format.md}}

{{#include time/duration.md}}

[Choosing between chrono, time and jiff]: duration.html#choosing-between-chrono-time-and-jiff
[`time`]: https://docs.rs/time/

{{#include ../links.md}}
