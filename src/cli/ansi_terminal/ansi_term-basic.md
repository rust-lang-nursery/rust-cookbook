## ANSI Terminal

[![ansi_term-badge]][ansi_term] [![cat-command-line-badge]][cat-command-line]

This program depicts the use of [`ansi_term` crate] and how it is used for controlling colours and formatting, such as blue bold text or yellow underlined text, on ANSI terminals.

There are two main data structures in this crate that you need to be concerned with: ANSIString and Style. A Style holds stylistic information: colours, whether the text should be bold, or blinking, or whatever. There are also Colour variants that represent simple foreground colour styles. An ANSIString is a string paired with a Style.

```rust
extern crate ansi_term;

// British english uses Colour instead of Color
use ansi_term::Colour;
use ansi_term::Style;

fn main() {
    println!("This is {} in color, {} in color and {} in color",
             Colour::Red.paint("red"),
             Colour::Blue.paint("blue"),
             Colour::Green.paint("green"));

    // The above can also be done using
    let blue_string = Colour::Blue.paint("Blue").to_string();
    println!("This is {} color", blue_string);

    // Bold text in the terminal
    println!("{} and this is not",
             Style::new().bold().paint("This is Bold"));

    // Bold and Color can also be combined
    println!("{}, {} and {}",
             Colour::Yellow.paint("This is colored"),
             Style::new().bold().paint("this is bold"),
             Colour::Yellow.bold().paint("this is bold and colored"));
}
```

[documentation]: https://docs.rs/ansi_term/
[`ansi_term` crate]: https://crates.io/crates/ansi_term