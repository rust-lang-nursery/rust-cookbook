use std::error::Error;

use pest::Parser;
use pest_derive::Parser;

/// `pest` derives a parser from a PEG grammar kept in a separate `.pest`
/// file. Each named rule in `csv.pest` becomes a `Rule` variant, and the
/// grammar — not hand-written code — describes what is valid.
#[derive(Parser)]
#[grammar = "csv.pest"]
struct CsvParser;

/// Parse a tiny CSV of numbers and return their sum. Walking the parse
/// tree is a matter of iterating `into_inner()` over the matched rules.
fn sum_csv(input: &str) -> Result<f64, Box<dyn Error>> {
    let file = CsvParser::parse(Rule::file, input)?
        .next()
        .ok_or("no file rule produced")?;

    let mut total = 0.0;
    for record in file.into_inner() {
        if record.as_rule() == Rule::record {
            for field in record.into_inner() {
                total += field.as_str().parse::<f64>()?;
            }
        }
    }
    Ok(total)
}

fn main() -> Result<(), Box<dyn Error>> {
    let total = sum_csv("1, 2, 3\n4, 5, 6")?;
    println!("sum = {total}");
    assert_eq!(total, 21.0);

    // pest errors carry line/column information and render a caret that
    // points at the offending input.
    if let Err(err) = sum_csv("1, 2\n3, x") {
        println!("{err}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_single_record() {
        assert_eq!(sum_csv("10, 20, 30").unwrap(), 60.0);
    }

    #[test]
    fn sums_multiple_records() {
        assert_eq!(sum_csv("1, 2, 3\n4, 5, 6").unwrap(), 21.0);
    }

    #[test]
    fn handles_negatives_and_decimals() {
        assert_eq!(sum_csv("-1.5, 2.5").unwrap(), 1.0);
    }

    #[test]
    fn rejects_non_numeric_field() {
        assert!(sum_csv("1, 2\n3, x").is_err());
    }
}
