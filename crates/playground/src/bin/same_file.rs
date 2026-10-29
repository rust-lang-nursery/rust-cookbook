fn main() -> std::io::Result<()> {
    println!(
        "same file: {}",
        same_file::is_same_file("/usr/bin/..", "/usr/")?
    );
    println!("same file: {}", same_file::is_same_file("/usr", "/tmp")?);
    Ok(())
}
