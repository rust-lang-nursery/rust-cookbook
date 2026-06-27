# Parse an unstructured string into key value pairs

[![winnow-badge]][winnow] [![cat-parser-implementations-badge]][cat-parser-implementations]

[`winnow`][winnow] is a parser-combinator library descended from `nom`, tuned
for ergonomics and speed. Its defining choice is that a parser takes `&mut &str`
and advances that slice in place, returning only the parsed value via
[`Parser::parse_next`]. Combinators such as [`separated_pair`] and [`separated`]
compose those functions into larger ones.

This recipe parses a settings string like `name=ferris; age=8; lang=rust` into a
list of key/value pairs. [`take_while`] collects identifier characters, and
[`Parser::parse`] runs the whole parser to completion. When the input is
malformed, the returned [`ParseError`] reports the byte offset where parsing
stopped — the foundation for location-aware diagnostics.

```rust,no_run
{{#include ../../crates/parsing/winnow/src/bin/winnow.rs::64}}
```

[`Parser::parse_next`]: https://docs.rs/winnow/*/winnow/trait.Parser.html#tymethod.parse_next
[`Parser::parse`]: https://docs.rs/winnow/*/winnow/trait.Parser.html#method.parse
[`separated_pair`]: https://docs.rs/winnow/*/winnow/combinator/fn.separated_pair.html
[`separated`]: https://docs.rs/winnow/*/winnow/combinator/fn.separated.html
[`take_while`]: https://docs.rs/winnow/*/winnow/token/fn.take_while.html
[`ParseError`]: https://docs.rs/winnow/*/winnow/error/struct.ParseError.html

{{#include ../links.md}}
