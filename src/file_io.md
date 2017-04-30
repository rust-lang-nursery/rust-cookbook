# File I/O Example

## Uses Rust's fs::File library to do simple file reading and writing operations

```rust
/**
 * Simple file I/O and operations.
 */

// File operations
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn operate_on_file(myfile: &str, mystring: &str, count: i32) -> io::Result<()> {

    // Try to open file
    let mut f = try!(File::open(myfile));

    // Write entire mystring buffer to file 'count' times
    let mut c = 0;
    while c < count {
        try!(f.write_all(mystring.as_bytes()));
        c += 1;
    }

    // Read lines from file
    for line in io::BufReader::new(&f).lines() {
        // FIXME: bad unwrap
        println!("{}", line.unwrap());
    }
    Ok(())
}

fn main() {

    let myfile = "input.txt";
    let mystring = "My favorite string! <3";
    let count = 10;

    match operate_on_file(&myfile, &mystring, count) {
        Ok(()) => { return; },
        Err(_) => { println!("File operations failed!")},
    };
}
```

The above program is a simple example of how to write and read to a file.
