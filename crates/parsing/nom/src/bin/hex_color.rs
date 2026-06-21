use std::error::Error;

use nom::bytes::complete::take_while_m_n;
use nom::character::complete::char;
use nom::combinator::map_res;
use nom::sequence::preceded;
use nom::{Finish, IResult, Parser};

/// An HTML-style `#1b2a3c` color literal decoded into its red, green and
/// blue components.
#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

/// `take_while_m_n` consumes between `m` and `n` characters matching a
/// predicate — here exactly two hex digits — and `map_res` turns them into
/// a byte, failing the parse instead of panicking on bad input.
fn hex_byte(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(2, 2, |c: char| c.is_ascii_hexdigit()),
        |hex| u8::from_str_radix(hex, 16),
    )
    .parse(input)
}

/// A leading `#` followed by three hex bytes.
fn color(input: &str) -> IResult<&str, Color> {
    // A tuple of parsers is itself a parser that runs each in sequence.
    let (input, (red, green, blue)) =
        preceded(char('#'), (hex_byte, hex_byte, hex_byte)).parse(input)?;
    Ok((input, Color { red, green, blue }))
}

fn main() -> Result<(), Box<dyn Error>> {
    // `Finish` converts the streaming-style result into a plain `Result`;
    // `?` then propagates a parse error instead of panicking.
    let (_, parsed) = color("#1b2a3c").finish()?;
    println!("{parsed:?}");
    assert_eq!(
        parsed,
        Color {
            red: 0x1b,
            green: 0x2a,
            blue: 0x3c
        }
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_color() {
        assert_eq!(
            color("#ffffff").finish(),
            Ok((
                "",
                Color {
                    red: 255,
                    green: 255,
                    blue: 255
                }
            ))
        );
    }

    #[test]
    fn rejects_non_hex() {
        assert!(color("#zzzzzz").finish().is_err());
    }
}
