## ANSI Terminal

[![ansi_term-badge]][ansi_term] [![cat-command-line-badge]][cat-command-line]

This program depicts the use of [`ansi_term` crate] and how it is used for controlling colours and formatting, such as blue bold text or yellow underlined text, on ANSI terminals.

There are two main data structures in this crate that our code needs to be concerned with: [`ANSIString`] and [`Style`]. A [`Style`] holds stylistic information: colours, whether the text should be bold, or blinking, or whatever. There are also Colour variants that represent simple foreground colour styles. An [`ANSIString`] is a string paired with a [`Style`].

**Note:** British english uses *Colour* instead of *Color*, don't get confused

### Printing colored text to the Terminal

```rust
extern crate ansi_term;

use ansi_term::Colour;

fn main() {
    println!("This is {} in color, {} in color and {} in color",
             Colour::Red.paint("red"),
             Colour::Blue.paint("blue"),
             Colour::Green.paint("green"));
}
```

#### Another way to print colored text to Terminal
The code can be reformatted such that it converts the [`ANSIString`] to a string as so any other `Display` value
```rust
extern crate ansi_term;

use ansi_term::Colour;

fn main() {
    let blue_string = Colour::Blue.paint("Blue").to_string();
    println!("This is {} color", blue_string);
}
```

### Bold text in Terminal
For anything more complex than plain foreground colour changes, the code needs to construct `Style` objects themselves, rather than beginning with a `Colour`. It can be achieved by chaining methods based on a new `Style`, created with [`Style::new()`]
```rust
extern crate ansi_term;

use ansi_term::Style;

fn main() {
    println!("{} and this is not",
             Style::new().bold().paint("This is Bold"));
}
```
### Bold and colored text in terminal
`Bold` methods can also been implemented for `Colour` values, and can be done by giving `styles` a foreground colour without having to begin with an empty Style value

```rust
extern crate ansi_term;

use ansi_term::Colour;
use ansi_term::Style;

fn main(){
    println!("{}, {} and {}",
             Colour::Yellow.paint("This is colored"),
             Style::new().bold().paint("this is bold"),
             Colour::Yellow.bold().paint("this is bold and colored"));
}
```

[documentation]: https://docs.rs/ansi_term/
[`ansi_term` crate]: https://crates.io/crates/ansi_term
[`ANSIString`]: https://docs.rs/ansi_term/*/ansi_term/type.ANSIString.html
[`Style`]: https://docs.rs/ansi_term/*/ansi_term/struct.Style.html
[`Style::new()`]: https://docs.rs/ansi_term/0.11.0/ansi_term/struct.Style.html#method.new