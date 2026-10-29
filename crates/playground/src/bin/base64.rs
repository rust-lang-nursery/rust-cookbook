use base64::Engine;
use base64::engine::general_purpose::STANDARD;

fn main() -> Result<(), base64::DecodeError> {
    let encoded = STANDARD.encode("hello, playground");
    println!("{encoded}");

    let decoded = STANDARD.decode(&encoded)?;
    println!("{}", String::from_utf8_lossy(&decoded));
    Ok(())
}
