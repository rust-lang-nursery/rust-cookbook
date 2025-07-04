```rust,edition2018
extern crate tempfile;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use tempfile::NamedTempFile;

fn main() -> io::Result<()> {
    // Create a temporary file with some content
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "Line 1")?;
    writeln!(temp_file, "Line 2")?;
    writeln!(temp_file, "Line 3")?;
    
    // Read lines from the file
    let file = File::open(temp_file.path())?;
    let reader = BufReader::new(file);
    
    for (index, line) in reader.lines().enumerate() {
        println!("Line {}: {}", index + 1, line?);
    }
    
    Ok(())
}
``` 