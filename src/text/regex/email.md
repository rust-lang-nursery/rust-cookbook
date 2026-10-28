## Verify and extract login from an email address

[![regex-badge]][regex] [![cat-text-processing-badge]][cat-text-processing]

Validates that an email address is formatted correctly, and extracts everything
before the @ symbol. The [`Regex`] is compiled only once, on first use, by
declaring it as a `static` [`LazyLock`].

```rust,edition2024
use regex::Regex;
use std::sync::LazyLock;

fn extract_login(input: &str) -> Option<&str> {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"(?x)
            ^(?P<login>[^@\s]+)@
            ([[:word:]]+\.)*
            [[:word:]]+$
            ",
        )
        .expect("valid regex")
    });
    RE.captures(input)
        .and_then(|cap| cap.name("login").map(|login| login.as_str()))
}

fn main() {
    assert_eq!(extract_login(r"I❤email@example.com"), Some(r"I❤email"));
    assert_eq!(
        extract_login(r"sdf+sdsfsd.as.sdsd@jhkk.d.rl"),
        Some(r"sdf+sdsfsd.as.sdsd")
    );
    assert_eq!(extract_login(r"More@Than@One@at.com"), None);
    assert_eq!(extract_login(r"Not an email@email"), None);
}
```

[`LazyLock`]: https://doc.rust-lang.org/std/sync/struct.LazyLock.html
[`Regex`]: https://docs.rs/regex/*/regex/struct.Regex.html
