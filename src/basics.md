# Basics

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Read lines of strings from a file][ex-std-read-lines] | [![std-badge]][std] | [![cat-file-io-badge]][cat-file-io] |
| [Read and write integers in little-endian byte order][ex-byteorder-le] | [![byteorder-badge]][byteorder] | [![cat-serialization-badge]][cat-serialization] |
| [Generate random floating point numbers][ex-rand-float] | [![rand-badge]][rand] | [![cat-math-badge]][cat-math] |
| [Generate random values on a custom type][ex-rand-custom] | [![rand-badge]][rand] | [![cat-math-badge]][cat-math] |
| [Construct a graph of objects][ex-petgraph-basic] | [![petgraph-badge]][petgraph] | [![cat-math-badge]][cat-math] |

<a name="ex-std-read-lines"></a>
## Read lines of strings from a file

[![std-badge]][std] [![cat-file-io-badge]][cat-file-io]

Writes a three-line message to a file, then reads it back a line at a
time with the [`Lines`] iterator created by
[`BufRead::lines`]. [`BufRead`] is a trait, and the most common way to
get one is from a [`BufReader`], which is constructed from some type
that implements [`Write`], here a [`File`]. The [`File`] is opened
for writing with [`File::create`], and reading with [`File::open`].

```rust
use std::fs::File;
use std::io::{self, Write, BufReader, BufRead};

fn run() -> io::Result<()> {

    write!(File::create("lines.txt")?, "Rust\n💖\nFun")?;

    let file = File::open("lines.txt")?;
    let file = BufReader::new(file);

    for line in file.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn main() { run().unwrap() }
```

<a name="ex-byteorder-le"></a>
## Read and write integers in little-endian byte order

[![byteorder-badge]][byteorder] [![cat-serialization-badge]][cat-serialization]

```rust
extern crate byteorder;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[derive(Default, Eq, PartialEq, Debug)]
struct Payload {
  kind: u8,
  value: u16,
}

fn run() -> Result<()> {
   let original_payload = Payload::default();
   let encoded_buf = encode(&original_payload)?;
   let decoded_payload = decode(&encoded_buf)?;
   assert_eq!(original_payload, decoded_payload);
   Ok(())
}

fn encode(payload: &Payload) -> Result<Vec<u8>> {
   let mut wtr = vec![];
   wtr.write_u8(payload.kind)?;
   wtr.write_u16::<LittleEndian>(payload.value)?;
   Ok(wtr)
}

fn decode(buf: &[u8]) -> Result<Payload> {
    let mut rdr = Cursor::new(buf);
    Ok(Payload {
        kind: rdr.read_u8()?,
        value: rdr.read_u16::<LittleEndian>()?,
    })
}

#[macro_use]
extern crate error_chain;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }
}

fn main() { run().unwrap() }
```

<a name="ex-rand-float"></a>
## Generate random floating point numbers

[![rand-badge]][rand] [![cat-math-badge]][cat-math]

<a name="ex-rand-custom"></a>
## Generate random values on a custom type

[![rand-badge]][rand] [![cat-math-badge]][cat-math]

<a name="ex-petgraph-basic"></a>
## Construct a graph of objects

[![petgraph-badge]][petgraph] [![cat-math-badge]][cat-math]

<!-- Categories -->

[cat-file-io-badge]: https://img.shields.io/badge/-file_io-orange.svg
[cat-file-io]: https://crates.io
[cat-math-badge]: https://img.shields.io/badge/-rand-orange.svg
[cat-math]: https://crates.io
[cat-serialization-badge]: https://img.shields.io/badge/-serialization-orange.svg
[cat-serialization]: https://crates.io

<!-- Crates -->

[byteorder-badge]: https://img.shields.io/badge/byteorder-1.0.0-blue.svg
[byteorder]: https://docs.rs/byteorder/1.0.0/byteorder/
[petgraph-badge]: https://img.shields.io/badge/petgraph-0.4.3-blue.svg
[petgraph]: https://docs.rs/petgraph/0.4.3/petgraph/
[std-badge]: https://img.shields.io/badge/std-1.17.0-blue.svg
[std]: https://doc.rust-lang.org/std
[rand-badge]: https://img.shields.io/badge/rand-0.3.15-blue.svg
[rand]: https://docs.rs/rand/0.3.15/rand/

<!-- Examples -->

[ex-byteorder-le]: basics.html#ex-byteorder-le
[ex-petgraph-basic]: basics.html#ex-petgraph-basic
[ex-rand-custom]: basics.html#ex-rand-custom
[ex-rand-float]: basics.html#ex-rand-float
[ex-std-read-lines]: basics.html#ex-std-read-lines

<!-- API links -->

[`BufRead::lines`]: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
[`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html
[`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`File::create`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.create
[`File::open`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.open
[`Lines`]: https://doc.rust-lang.org/std/io/struct.Lines.html
[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
