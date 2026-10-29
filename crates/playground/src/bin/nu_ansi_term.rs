use nu_ansi_term::{Color, Style};

fn main() {
    println!(
        "{} {} {}",
        Color::Red.paint("red"),
        Color::Green.paint("green"),
        Color::Blue.paint("blue")
    );
    println!(
        "{}",
        Style::new().bold().underline().paint("bold and underlined")
    );
}
