use memmap2::Mmap;
use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    write!(
        File::create("content.txt")?,
        "My hovercraft is full of eels!"
    )?;

    let file = File::open("content.txt")?;
    // SAFETY: nothing else modifies or truncates `content.txt` while it is mapped.
    let map = unsafe { Mmap::map(&file)? };

    let random_indexes = [0, 1, 2, 19, 22, 10, 11, 29];
    assert_eq!(&map[3..13], b"hovercraft");
    let random_bytes: Vec<u8> = random_indexes.iter().map(|&idx| map[idx]).collect();
    assert_eq!(&random_bytes[..], b"My loaf!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_and_reads_random_bytes() -> Result<(), Error> {
        main()?;
        std::fs::remove_file("content.txt")
    }
}
