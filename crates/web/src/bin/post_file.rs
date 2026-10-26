use anyhow::Result;
use std::fs::File;
use std::io::Read;

fn main() -> Result<()> {
    let paste_api = "https://paste.rs";
    let mut file = File::open("message")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let client = reqwest::blocking::Client::new();
    let res = client.post(paste_api).body(contents).send()?;
    let response_text = res.text()?;
    println!("Your paste is located at: {}", response_text);
    Ok(())
}
