## ANSI Terminal

[![nu-ansi-term-badge]][nu-ansi-term] [![cat-command-line-badge]][cat-command-line]

This program depicts the use of [`nu-ansi-term`] crate and how it is used for controlling colors and formatting, such as blue bold text or yellow underlined text, on ANSI terminals. [`nu-ansi-term`] is a maintained fork of the archived `ansi_term` crate with a nearly identical API.

There are two main data structures in [`nu-ansi-term`]: [`AnsiString`] and [`Style`]. A [`Style`] holds stylistic information: colors, whether the text should be bold, or blinking, or whatever. There are also [`Color`] variants that represent simple foreground color styles. An [`AnsiString`] is a string paired with a [`Style`].

### Printing colored text to the Terminal

```rust,edition2024
{{#include ../../../crates/cli/ansi_terminal/src/bin/colors.rs::10 }}
```

### Bold text in Terminal

For anything more complex than plain foreground color changes, the code
needs to construct `Style` struct. [`Style::new()`] creates the struct,
and properties chained.

```rust,edition2024
{{#include ../../../crates/cli/ansi_terminal/src/bin/bold.rs::8 }}
```

### Bold and colored text in terminal

`Color` implements many similar functions as `Style` and can chain methods.

```rust,edition2024
{{#include ../../../crates/cli/ansi_terminal/src/bin/bold_colors.rs::11 }}
```

[`nu-ansi-term`]: https://docs.rs/nu-ansi-term/*/nu_ansi_term/
[`AnsiString`]: https://docs.rs/nu-ansi-term/*/nu_ansi_term/type.AnsiString.html
[`Color`]: https://docs.rs/nu-ansi-term/*/nu_ansi_term/enum.Color.html
[`Style`]: https://docs.rs/nu-ansi-term/*/nu_ansi_term/struct.Style.html
[`Style::new()`]: https://docs.rs/nu-ansi-term/*/nu_ansi_term/struct.Style.html#method.new
