## Avoid writing and reading from a same file

[![same_file-badge]][same_file] [![cat-filesystem-badge]][cat-filesystem]

Use [`same_file::Handle`] to a file that can be tested for equality with
other handles. In this example, the handles of file to be read from and
to be written to are tested for equality.

```rust,no_run
extern crate same_file;

use same_file::Handle;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

fn main() -> Result<(), Error> {
    let path_to_read = Path::new("new.txt");

    let stdout_handle = Handle::stdout()?;
    let handle = Handle::from_path(path_to_read)?;

    if stdout_handle == handle {
        return Err(Error::new(
            ErrorKind::Other,
            "You are reading and writing to the same file",
        ));
    } else {
        let file = File::open(&path_to_read)?;
        let file = BufReader::new(file);
        for (num, line) in file.lines().enumerate() {
            println!("{} : {}", num, line?.to_uppercase());
        }
    }

    Ok(())
}
```

```bash
cargo run
```
displays the contents of the file new.txt.

```bash
cargo run >> ./new.txt
```
errors because the two files are same.

[`same_file::Handle`]: https://docs.rs/same-file/*/same_file/struct.Handle.html
