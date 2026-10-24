## Extract a list of unique #Hashtags from a text

[![regex-badge]][regex] [![std-badge]][std] [![cat-text-processing-badge]][cat-text-processing]

Extracts, sorts, and deduplicates list of hashtags from text.

The hashtag regex given here only catches Latin hashtags that start with a
letter. The complete [twitter hashtag regex] is much more complicated.

The [`Regex`] is compiled only once, on first use, by declaring it as a
`static` [`LazyLock`].

```rust,edition2021
use regex::Regex;
use std::collections::HashSet;
use std::sync::LazyLock;

fn extract_hashtags(text: &str) -> HashSet<&str> {
    static HASHTAG_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"\#[a-zA-Z][0-9a-zA-Z_]*").expect("valid regex"));
    HASHTAG_REGEX
        .find_iter(text)
        .map(|mat| mat.as_str())
        .collect()
}

fn main() {
    let tweet = "Hey #world, I just got my new #dog, say hello to Till. #dog #forever #2 #_ ";
    let tags = extract_hashtags(tweet);
    assert!(tags.contains("#dog") && tags.contains("#forever") && tags.contains("#world"));
    assert_eq!(tags.len(), 3);
}
```

[`LazyLock`]: https://doc.rust-lang.org/std/sync/struct.LazyLock.html
[`Regex`]: https://docs.rs/regex/*/regex/struct.Regex.html
[twitter hashtag regex]: https://github.com/twitter/twitter-text/blob/c9fc09782efe59af4ee82855768cfaf36273e170/java/src/com/twitter/Regex.java#L255
