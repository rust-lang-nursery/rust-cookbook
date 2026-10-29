use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("could not parse {0:?} as a number")]
    Parse(String, #[source] std::num::ParseIntError),
    #[error("{0} is too large (max 100)")]
    TooLarge(u32),
}

fn parse(input: &str) -> Result<u32, AppError> {
    let n: u32 = input
        .parse()
        .map_err(|e| AppError::Parse(input.to_owned(), e))?;
    if n > 100 {
        return Err(AppError::TooLarge(n));
    }
    Ok(n)
}

fn main() {
    for input in ["42", "abc", "500"] {
        match parse(input) {
            Ok(n) => println!("ok: {n}"),
            Err(err) => println!("error: {err}"),
        }
    }
}
