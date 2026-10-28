## Parse and format a datetime with a time zone

[![jiff-badge]][jiff] [![cat-date-and-time-badge]][cat-date-and-time]

Use `jiff` when the time zone name must survive a round trip through text;
see [Choosing between chrono, time and jiff]. Parses an RFC 9557 string, which includes an IANA time zone name
(`[America/New_York]`) as well as the offset, into a [`Zoned`] using
[`FromStr`]. Output is formatted with [`Zoned::strftime`], which can print the
zone abbreviation (`EDT`), and [`strtime::parse`] handles custom patterns.
Invalid input returns an `Err`.

```rust
{{#include ../../../crates/datetime/jiff/src/bin/parse_format.rs::22 }}
```

[`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
[`strtime::parse`]: https://docs.rs/jiff/*/jiff/fmt/strtime/fn.parse.html
[`Zoned`]: https://docs.rs/jiff/*/jiff/struct.Zoned.html
[`Zoned::strftime`]: https://docs.rs/jiff/*/jiff/struct.Zoned.html#method.strftime
