use nu_ansi_term::Style;

fn main() {
    println!(
        "{} and this is not",
        Style::new().bold().paint("This is Bold")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs() {
        main();
    }

    #[test]
    fn wraps_text_in_escape_codes() {
        assert_eq!(
            Style::new().bold().paint("bold").to_string(),
            "\u{1b}[1mbold\u{1b}[0m"
        );
    }
}
