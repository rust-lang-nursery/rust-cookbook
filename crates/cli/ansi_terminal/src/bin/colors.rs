use nu_ansi_term::Color;

fn main() {
    println!(
        "This is {} in color, {} in color and {} in color",
        Color::Red.paint("red"),
        Color::Blue.paint("blue"),
        Color::Green.paint("green")
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
            Color::Red.paint("red").to_string(),
            "\u{1b}[31mred\u{1b}[0m"
        );
    }
}
