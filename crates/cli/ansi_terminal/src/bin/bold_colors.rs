use nu_ansi_term::Color;
use nu_ansi_term::Style;

fn main() {
    println!(
        "{}, {} and {}",
        Color::Yellow.paint("This is colored"),
        Style::new().bold().paint("this is bold"),
        Color::Yellow.bold().paint("this is bold and colored")
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
    fn combines_bold_and_color() {
        assert_eq!(
            Color::Yellow.bold().paint("both").to_string(),
            "\u{1b}[1;33mboth\u{1b}[0m"
        );
    }
}
