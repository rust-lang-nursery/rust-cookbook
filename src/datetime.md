# Date and Time

Not sure which crate to use? See [Choosing between chrono, time and jiff][ex-choosing-datetime-crate].

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Measure elapsed time][ex-measure-elapsed-time] | [![std-badge]][std] | [![cat-time-badge]][cat-time] |
| [Deadline arithmetic with Duration][ex-deadline-arithmetic-std] | [![std-badge]][std] | [![cat-time-badge]][cat-time] |
| [Convert SystemTime to UNIX timestamp][ex-convert-to-unix-timestamp-std] | [![std-badge]][std] | [![cat-time-badge]][cat-time] |
| [Benchmark a closure with Instant][ex-benchmark-closure] | [![std-badge]][std] | [![cat-time-badge]][cat-time] |
| [Perform checked date and time calculations][ex-datetime-arithmetic] | [![chrono-badge]][chrono] | [![cat-date-and-time-badge]][cat-date-and-time] |
| [Convert a local time to another timezone][ex-convert-datetime-timezone] | [![chrono-badge]][chrono] | [![cat-date-and-time-badge]][cat-date-and-time] |
| [Examine the date and time][ex-examine-date-and-time] | [![chrono-badge]][chrono] | [![cat-date-and-time-badge]][cat-date-and-time] |
| [Convert date to UNIX timestamp and vice versa][ex-convert-datetime-timestamp] | [![chrono-badge]][chrono] | [![cat-date-and-time-badge]][cat-date-and-time] |
| [Display formatted date and time][ex-format-datetime] | [![chrono-badge]][chrono] | [![cat-date-and-time-badge]][cat-date-and-time] |
| [Parse string into DateTime struct][ex-parse-datetime] | [![chrono-badge]][chrono] | [![cat-date-and-time-badge]][cat-date-and-time] |
| [Examine the current time with a time zone][ex-jiff-now] | [![jiff-badge]][jiff] | [![cat-date-and-time-badge]][cat-date-and-time] |
| [Parse and format a datetime with a time zone][ex-jiff-parse-format] | [![jiff-badge]][jiff] | [![cat-date-and-time-badge]][cat-date-and-time] |
| [Perform calendar arithmetic with Span][ex-jiff-span] | [![jiff-badge]][jiff] | [![cat-date-and-time-badge]][cat-date-and-time] |
| [Examine the current time with a UTC offset][ex-time-now] | [![time-badge]][time] | [![cat-date-and-time-badge]][cat-date-and-time] |
| [Parse and format a datetime with format descriptions][ex-time-parse-format] | [![time-badge]][time] | [![cat-date-and-time-badge]][cat-date-and-time] |
| [Perform checked date and time calculations with Duration][ex-time-duration] | [![time-badge]][time] | [![cat-date-and-time-badge]][cat-date-and-time] |

[ex-measure-elapsed-time]: datetime/duration.html#measure-the-elapsed-time-between-two-code-sections
[ex-deadline-arithmetic-std]: datetime/duration.html#deadline-arithmetic-with-instant
[ex-convert-to-unix-timestamp-std]: datetime/duration.html#convert-systemtime-to-unix-timestamp
[ex-benchmark-closure]: datetime/duration.html#benchmark-a-closure-with-instant
[ex-datetime-arithmetic]: datetime/duration.html#perform-checked-date-and-time-calculations
[ex-convert-datetime-timezone]: datetime/duration.html#convert-a-local-time-to-another-timezone
[ex-examine-date-and-time]: datetime/parse.html#examine-the-date-and-time
[ex-convert-datetime-timestamp]: datetime/parse.html#convert-date-to-unix-timestamp-and-vice-versa
[ex-format-datetime]: datetime/parse.html#display-formatted-date-and-time
[ex-parse-datetime]: datetime/parse.html#parse-string-into-datetime-struct
[ex-jiff-now]: datetime/jiff.html#examine-the-current-time-with-a-time-zone
[ex-jiff-parse-format]: datetime/jiff.html#parse-and-format-a-datetime-with-a-time-zone
[ex-jiff-span]: datetime/jiff.html#perform-calendar-arithmetic-with-span
[ex-time-now]: datetime/time.html#examine-the-current-time-with-a-utc-offset
[ex-time-parse-format]: datetime/time.html#parse-and-format-a-datetime-with-format-descriptions
[ex-time-duration]: datetime/time.html#perform-checked-date-and-time-calculations-with-duration
[ex-choosing-datetime-crate]: datetime/duration.html#choosing-between-chrono-time-and-jiff

{{#include links.md}}
