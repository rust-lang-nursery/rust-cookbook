## Use Cursor as an in-memory buffer

[![std-badge]][std] [![cat-filesystem-badge]][cat-filesystem]

[`Cursor`] wraps an in-memory buffer and implements [`io::Read`],
[`io::Write`], and [`io::Seek`], making it a drop-in replacement for a
[`File`] in tests or whenever bytes need to be assembled before deciding
where to send them. The internal position advances with each read or write,
so call [`Cursor::set_position`] to rewind before reading back what you wrote.

```rust,edition2021
use std::io::{self, Cursor, Read, Write};

fn main() -> io::Result<()> {
    let mut buf = Cursor::new(Vec::<u8>::new());

    // Write into the buffer exactly as you would a File.
    write!(buf, "hello, ")?;
    write!(buf, "world")?;

    // Rewind to the start before reading back.
    buf.set_position(0);
    let mut out = String::new();
    buf.read_to_string(&mut out)?;

    assert_eq!(out, "hello, world");
    println!("{out}");
    Ok(())
}
```

[`Cursor`]: https://doc.rust-lang.org/std/io/struct.Cursor.html
[`Cursor::set_position`]: https://doc.rust-lang.org/std/io/struct.Cursor.html#method.set_position
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`io::Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`io::Seek`]: https://doc.rust-lang.org/std/io/trait.Seek.html
[`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
