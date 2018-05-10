## Read lines of strings from a file

[![std-badge]][std] [![cat-filesystem-badge]][cat-filesystem]

Writes a three-line message to a file, then reads it back a line at a
time with the [`Lines`] iterator created by
[`BufRead::lines`]. [`BufRead`] is a trait, and the most common way to
get one is from a [`BufReader`], which is constructed from some type
that implements [`Read`], here a [`File`]. The [`File`] is opened
for writing with [`File::create`], and reading with [`File::open`].

```rust
# #[macro_use]
# extern crate error_chain;
#
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
#
# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#     }
# }

fn run() -> Result<()> {
    let path = "lines.txt";

    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFun")?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}
#
# quick_main!(run);
```

[`BufRead::lines`]: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
[`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html
[`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
[`File::create`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.create
[`File::open`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.open
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`Lines`]: https://doc.rust-lang.org/std/io/struct.Lines.html
[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
