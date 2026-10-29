use mime::Mime;

fn main() -> Result<(), mime::FromStrError> {
    let m: Mime = "text/html; charset=utf-8".parse()?;
    println!("type: {}, subtype: {}", m.type_(), m.subtype());
    println!("charset: {:?}", m.get_param(mime::CHARSET));
    println!(
        "is html: {}",
        m.essence_str() == mime::TEXT_HTML.essence_str()
    );
    Ok(())
}
