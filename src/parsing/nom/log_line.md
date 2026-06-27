## Parse tagged fields from a log line

[![nom-badge]][nom] [![cat-parser-implementations-badge]][cat-parser-implementations]

[`nom`][nom] is a parser-combinator library: you build a parser by composing
small parsers, each of which consumes part of the input and hands the rest to
the next. A parser is any function `fn(Input) -> IResult<Input, Output>`, where
[`IResult`] carries either the parsed value together with the unconsumed tail,
or an error.

This recipe parses a log line like `level=warn line=42` into a struct. Tokens
are matched with [`tag`] and alternatives with [`alt`]; [`value`] maps each
matched keyword to a `Level` without a closure. A fallible conversion (string to
`u32`) is wrapped in [`map_res`] so a bad number becomes a parse error rather
than a panic, and [`Finish`] turns the streaming-style result into a plain
`Result` once parsing is complete.

```rust
{{#include ../../../crates/parsing/nom/src/bin/log_line.rs::73}}
```

[`IResult`]: https://docs.rs/nom/*/nom/type.IResult.html
[`tag`]: https://docs.rs/nom/*/nom/bytes/complete/fn.tag.html
[`alt`]: https://docs.rs/nom/*/nom/branch/fn.alt.html
[`value`]: https://docs.rs/nom/*/nom/combinator/fn.value.html
[`map_res`]: https://docs.rs/nom/*/nom/combinator/fn.map_res.html
[`Finish`]: https://docs.rs/nom/*/nom/trait.Finish.html
