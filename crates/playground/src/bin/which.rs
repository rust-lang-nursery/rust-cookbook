fn main() -> Result<(), which::Error> {
    println!("sh is at {}", which::which("sh")?.display());
    for path in which::which_all("cargo")? {
        println!("cargo: {}", path.display());
    }
    Ok(())
}
