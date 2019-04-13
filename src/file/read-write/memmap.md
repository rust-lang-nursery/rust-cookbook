## Access a file randomly using a memory map

[![memmap-badge]][memmap] [![cat-filesystem-badge]][cat-filesystem]

Creates a memory map of a file using [memmap] and simulates some non-sequential
reads from the file. Using a memory map means you just index into a slice rather
than dealing with [`seek`] to navigate a File.

The [`Mmap::map`] function assumes the file
behind the memory map is not being modified at the same time by another process
or else a [race condition] occurs.

```rust
extern crate memmap;

use memmap::Mmap;
use std::fs::File;
use std::io::{Write, Error};

fn main() -> Result<(), Error> {
#     write!(File::create("content.txt")?, "My hovercraft is full of eels!")?;
#
    let file = File::open("content.txt")?;
    let map = unsafe { Mmap::map(&file)? };

    let random_indexes = [0, 1, 2, 19, 22, 10, 11, 29];
    assert_eq!(&map[3..13], b"hovercraft");
    let random_bytes: Vec<u8> = random_indexes.iter()
        .map(|&idx| map[idx])
        .collect();
    assert_eq!(&random_bytes[..], b"My loaf!");
    Ok(())
}
```

[`Mmap::map`]: https://docs.rs/memmap/*/memmap/struct.Mmap.html#method.map
[`seek`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.seek

[race condition]: https://en.wikipedia.org/wiki/Race_condition#File_systems
