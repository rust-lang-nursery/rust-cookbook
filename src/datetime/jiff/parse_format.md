## Parse and format a datetime with a time zone

[![jiff-badge]][jiff] [![cat-date-and-time-badge]][cat-date-and-time]

Parses an RFC 9557 string, which includes an IANA time zone name, into a
[`Zoned`] using [`FromStr`]. Output is formatted with [`Zoned::strftime`], and
[`strtime::parse`] handles custom patterns. Invalid input returns an `Err`.

```rust
{{#include ../../../crates/datetime/jiff/src/bin/parse_format.rs }}
```

[`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
[`strtime::parse`]: https://docs.rs/jiff/*/jiff/fmt/strtime/fn.parse.html
[`Zoned`]: https://docs.rs/jiff/*/jiff/struct.Zoned.html
[`Zoned::strftime`]: https://docs.rs/jiff/*/jiff/struct.Zoned.html#method.strftime
