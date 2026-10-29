use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
word = { ASCII_ALPHA+ }
sentence = { SOI ~ word ~ (" " ~ word)* ~ EOI }
"#]
struct SentenceParser;

fn main() -> Result<(), Box<pest::error::Error<Rule>>> {
    let pairs = SentenceParser::parse(Rule::sentence, "hello pest world").map_err(Box::new)?;
    for pair in pairs.flatten() {
        if pair.as_rule() == Rule::word {
            println!("word: {}", pair.as_str());
        }
    }

    if let Err(err) = SentenceParser::parse(Rule::sentence, "no digits 42") {
        println!("{err}");
    }
    Ok(())
}
