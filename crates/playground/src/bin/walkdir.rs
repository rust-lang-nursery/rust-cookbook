use walkdir::WalkDir;

fn main() -> Result<(), walkdir::Error> {
    for entry in WalkDir::new("/usr").max_depth(1) {
        println!("{}", entry?.path().display());
    }
    Ok(())
}
