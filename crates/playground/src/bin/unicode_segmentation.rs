use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let text = "a\u{310}e\u{301}o\u{308}\u{332} \u{1F1F7}\u{1F1FA}\u{1F1F8}\u{1F1F9}";
    println!(
        "{} chars, {} graphemes",
        text.chars().count(),
        text.graphemes(true).count()
    );

    for word in "The quick (\"brown\") fox".unicode_words() {
        println!("{word}");
    }
}
