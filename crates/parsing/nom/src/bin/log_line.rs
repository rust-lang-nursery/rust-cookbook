use std::error::Error;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::combinator::{map, map_res, value};
use nom::sequence::{preceded, separated_pair};
use nom::{Finish, IResult, Parser};

/// A single log line such as `level=warn line=42` parsed into a struct.
#[derive(Debug, PartialEq)]
struct LogEntry {
    level: Level,
    line: u32,
}

#[derive(Clone, Debug, PartialEq)]
enum Level {
    Info,
    Warn,
    Error,
}

/// `nom` builds a parser by combining small parsers. Each one takes the
/// remaining input and returns the parsed value plus the unconsumed tail,
/// so combinators like `separated_pair` and `preceded` thread that tail
/// through for you. `parse` drives one of these combinators over the input.
///
/// `value` matches a token and yields a fixed value, ignoring the matched
/// text — exactly what mapping each keyword to a `Level` needs.
fn level(input: &str) -> IResult<&str, Level> {
    alt((
        value(Level::Info, tag("info")),
        value(Level::Warn, tag("warn")),
        value(Level::Error, tag("error")),
    ))
    .parse(input)
}

/// `map_res` runs a fallible conversion (`str::parse`) and turns an `Err`
/// into a parse failure instead of panicking.
fn number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse).parse(input)
}

/// `level=<level> line=<number>`, separated by whitespace.
fn log_entry(input: &str) -> IResult<&str, LogEntry> {
    map(
        separated_pair(
            preceded(tag("level="), level),
            space1,
            preceded(tag("line="), number),
        ),
        |(level, line)| LogEntry { level, line },
    )
    .parse(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    // `Finish` converts the streaming-style result into a plain `Result`;
    // `?` then propagates a parse error instead of panicking. The empty
    // remaining input is discarded.
    let (_, entry) = log_entry("level=warn line=42").finish()?;
    println!("{entry:?}");
    assert_eq!(
        entry,
        LogEntry {
            level: Level::Warn,
            line: 42
        }
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_each_level() {
        assert_eq!(level("info"), Ok(("", Level::Info)));
        assert_eq!(level("warn"), Ok(("", Level::Warn)));
        assert_eq!(level("error"), Ok(("", Level::Error)));
    }

    #[test]
    fn parses_full_entry() {
        assert_eq!(
            log_entry("level=error line=7").finish(),
            Ok((
                "",
                LogEntry {
                    level: Level::Error,
                    line: 7
                }
            ))
        );
    }
}
