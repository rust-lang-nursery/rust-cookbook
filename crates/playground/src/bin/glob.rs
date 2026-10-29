fn main() -> Result<(), Box<dyn std::error::Error>> {
    for entry in glob::glob("/usr/*")?.take(10) {
        println!("{}", entry?.display());
    }
    Ok(())
}
