use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::{IResult, Parser};

/// Parses `v<digits>`, e.g. `v42`, and returns the number.
fn version(input: &str) -> IResult<&str, u32> {
    let (rest, _) = tag("v").parse(input)?;
    let (rest, digits) = digit1.parse(rest)?;
    Ok((rest, digits.parse().unwrap_or(0)))
}

fn main() {
    println!("{:?}", version("v42 and more"));
    println!("{:?}", version("nope"));
}
