use tar::{Archive, Builder, Header};

fn main() -> std::io::Result<()> {
    let contents = b"hello, tar";
    let mut header = Header::new_gnu();
    header.set_size(contents.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();

    let mut builder = Builder::new(Vec::new());
    builder.append_data(&mut header, "hello.txt", &contents[..])?;
    let bytes = builder.into_inner()?;

    for entry in Archive::new(bytes.as_slice()).entries()? {
        let entry = entry?;
        println!("{} ({} bytes)", entry.path()?.display(), entry.size());
    }
    Ok(())
}
