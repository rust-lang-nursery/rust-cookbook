use data_encoding::{BASE32, HEXLOWER};

fn main() -> Result<(), data_encoding::DecodeError> {
    let hex = HEXLOWER.encode(b"playground");
    println!("hex:    {hex}");
    println!("base32: {}", BASE32.encode(b"playground"));

    let bytes = HEXLOWER.decode(hex.as_bytes())?;
    println!("back:   {}", String::from_utf8_lossy(&bytes));
    Ok(())
}
