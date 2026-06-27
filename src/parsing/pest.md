# Read CSV data with a parsing expression grammar

[![pest-badge]][pest] [![cat-parser-implementations-badge]][cat-parser-implementations]

[`pest`][pest] takes a different approach from the combinator libraries: instead
of writing parser code, you describe the language in a [Parsing Expression
Grammar] kept in a separate `.pest` file, and [`pest_derive`] generates the
parser at compile time. Each named rule becomes a variant of a generated `Rule`
enum.

The grammar below recognises a tiny CSV of numbers:

```text
{{#include ../../crates/parsing/pest/src/csv.pest}}
```

This recipe feeds input to the derived `CsvParser`, walks the resulting parse
tree with [`Pairs::into_inner`], and sums every field. Because the grammar
encodes what is valid, malformed input produces a [`pest::error::Error`] that
renders a caret pointing at the exact line and column — note the `?` operator
propagates it like any other error, with no `unwrap`.

```rust,no_run
{{#include ../../crates/parsing/pest/src/bin/pest.rs::42}}
```

Parsing `1, 2\n3, x` prints a located error:

```text
 --> 2:4
  |
2 | 3, x
  |    ^---
  |
  = expected field
```

[Parsing Expression Grammar]: https://docs.rs/pest/*/pest/index.html
[`pest_derive`]: https://docs.rs/pest_derive/*/pest_derive/
[`Pairs::into_inner`]: https://docs.rs/pest/*/pest/iterators/struct.Pair.html#method.into_inner
[`pest::error::Error`]: https://docs.rs/pest/*/pest/error/struct.Error.html

{{#include ../links.md}}
