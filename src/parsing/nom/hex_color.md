## Decode a hex color

[![nom-badge]][nom] [![cat-parser-implementations-badge]][cat-parser-implementations]

A second [`nom`][nom] parser, this one working at the byte level: it decodes an
HTML-style `#1b2a3c` color literal into its red, green and blue components.

[`take_while_m_n`] consumes a fixed number of characters matching a predicate —
here exactly two hex digits — and [`map_res`] converts each pair into a `u8`,
failing the parse instead of panicking on invalid input. A tuple of parsers,
`(hex_byte, hex_byte, hex_byte)`, is itself a parser that runs each in sequence,
and [`preceded`] discards the leading `#`. [`Finish`] turns the streaming-style
result into a plain `Result` once parsing is complete.

```rust
{{#include ../../../crates/parsing/nom/src/bin/hex_color.rs::51}}
```

[`take_while_m_n`]: https://docs.rs/nom/*/nom/bytes/complete/fn.take_while_m_n.html
[`map_res`]: https://docs.rs/nom/*/nom/combinator/fn.map_res.html
[`preceded`]: https://docs.rs/nom/*/nom/sequence/fn.preceded.html
[`Finish`]: https://docs.rs/nom/*/nom/trait.Finish.html
