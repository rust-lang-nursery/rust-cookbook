## Replace all occurrences of one text pattern with another pattern.

[![regex-badge]][regex] [![std-badge]][std] [![cat-text-processing-badge]][cat-text-processing]

Replaces all occurrences of the standard ISO 8601 *YYYY-MM-DD* date pattern
with the equivalent American English date with slashes.
For example `2013-01-15` becomes `01/15/2013`.

The method [`Regex::replace_all`] replaces all occurrences of the whole regex.
`&str` implements the `Replacer` trait which allows variables like `$abcde` to
refer to corresponding named capture groups `(?P<abcde>REGEX)` from the search
regex. See the [replacement string syntax] for examples and escaping detail.
The regex is compiled only once, on first use, by declaring it as a `static`
[`LazyLock`].

```rust,edition2024
use regex::Regex;
use std::borrow::Cow;
use std::sync::LazyLock;

fn reformat_dates(before: &str) -> Cow<str> {
    static ISO8601_DATE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").expect("valid regex")
    });
    ISO8601_DATE_REGEX.replace_all(before, "$m/$d/$y")
}

fn main() {
    let before = "2012-03-14, 2013-01-15 and 2014-07-05";
    let after = reformat_dates(before);
    assert_eq!(after, "03/14/2012, 01/15/2013 and 07/05/2014");
}
```

[`LazyLock`]: https://doc.rust-lang.org/std/sync/struct.LazyLock.html
[`Regex::replace_all`]: https://docs.rs/regex/*/regex/struct.Regex.html#method.replace_all

[replacement string syntax]: https://docs.rs/regex/*/regex/struct.Regex.html#replacement-string-syntax
