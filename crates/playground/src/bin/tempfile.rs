use std::fs;

fn main() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("note.txt");
    fs::write(&path, "gone when `dir` is dropped")?;
    println!("{}: {}", path.display(), fs::read_to_string(&path)?);
    Ok(())
}
