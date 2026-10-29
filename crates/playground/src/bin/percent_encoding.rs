use percent_encoding::{NON_ALPHANUMERIC, percent_decode_str, utf8_percent_encode};

fn main() -> Result<(), std::str::Utf8Error> {
    let encoded = utf8_percent_encode("rust cookbook/100%", NON_ALPHANUMERIC).to_string();
    println!("{encoded}");
    println!("{}", percent_decode_str(&encoded).decode_utf8()?);
    Ok(())
}
