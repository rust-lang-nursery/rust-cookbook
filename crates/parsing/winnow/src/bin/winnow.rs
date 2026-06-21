use winnow::ascii::space0;
use winnow::combinator::{separated, separated_pair};
use winnow::error::{ContextError, ParseError};
use winnow::prelude::*;
use winnow::token::take_while;

#[derive(Debug, PartialEq)]
struct Pair {
    key: String,
    value: String,
}

/// In `winnow` a parser is a function over `&mut &str`: it advances the
/// input slice in place and returns just the parsed value. `parse_next`
/// drives a single parser, and combinators like `separated_pair` compose
/// several into a larger one.
fn identifier(input: &mut &str) -> ModalResult<String> {
    take_while(1.., |c: char| c.is_alphanumeric() || c == '_')
        .map(|s: &str| s.to_string())
        .parse_next(input)
}

/// `key=value`, discarding the `=` separator.
fn pair(input: &mut &str) -> ModalResult<Pair> {
    separated_pair(identifier, '=', identifier)
        .map(|(key, value)| Pair { key, value })
        .parse_next(input)
}

/// One or more `pair`s separated by `;` and optional spaces.
fn pairs(input: &mut &str) -> ModalResult<Vec<Pair>> {
    separated(1.., pair, (space0, ';', space0)).parse_next(input)
}

/// `Parser::parse` runs a parser to completion over the whole input. On
/// failure it returns a `ParseError` that knows the byte offset where
/// parsing stopped, which is what makes good diagnostics possible.
fn parse_settings(input: &str) -> Result<Vec<Pair>, ParseError<&str, ContextError>> {
    pairs.parse(input)
}

fn main() {
    // winnow's `ParseError` isn't a `std::error::Error`, so it is handled
    // explicitly rather than with `?`.
    match parse_settings("name=ferris; age=8; lang=rust") {
        Ok(settings) => {
            println!("{settings:?}");
            assert_eq!(settings.len(), 3);
            assert_eq!(
                settings[0],
                Pair {
                    key: "name".to_string(),
                    value: "ferris".to_string()
                }
            );
        }
        Err(err) => eprintln!("unexpected parse error at byte offset {}", err.offset()),
    }

    // On malformed input the error reports exactly where parsing failed.
    if let Err(err) = parse_settings("name=ferris;=oops") {
        println!("parse failed at byte offset {}", err.offset());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_pair() {
        let mut input = "host=localhost";
        assert_eq!(
            pair(&mut input).unwrap(),
            Pair {
                key: "host".to_string(),
                value: "localhost".to_string()
            }
        );
    }

    #[test]
    fn parses_multiple_pairs() {
        let parsed = parse_settings("a=1;b=2;c=3").unwrap();
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[2].value, "3");
    }

    #[test]
    fn tolerates_spaces_around_separator() {
        let parsed = parse_settings("a=1 ; b=2").unwrap();
        assert_eq!(parsed.len(), 2);
    }

    #[test]
    fn reports_offset_on_failure() {
        let err = parse_settings("a=1;=2").unwrap_err();
        assert_eq!(err.offset(), 3);
    }
}
