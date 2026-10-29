use std::fs::File;
use std::io::Write;

use memmap2::Mmap;

fn main() -> std::io::Result<()> {
    let path = std::env::temp_dir().join("playground-mmap.txt");
    File::create(&path)?.write_all(b"My hovercraft is full of eels!")?;

    let file = File::open(&path)?;
    // SAFETY: nothing else modifies or truncates the file while it is mapped.
    let map = unsafe { Mmap::map(&file)? };
    println!("{}", String::from_utf8_lossy(&map[3..13]));
    Ok(())
}
