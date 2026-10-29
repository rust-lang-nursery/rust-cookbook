use exif::Reader;

fn main() {
    // An EXIF block normally comes from a JPEG; `Reader::read_from_container` reads a whole file.
    // Empty input is not valid EXIF, so this prints the error.
    match Reader::new().read_raw(Vec::new()) {
        Ok(exif) => println!("{} fields", exif.fields().count()),
        Err(err) => println!("no EXIF data: {err}"),
    }
}
