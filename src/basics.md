# Basics

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Read lines of strings from a file][ex-std-read-lines] | [![std-badge]][std] | [![cat-filesystem-badge]][cat-filesystem] |
| [Read and write integers in little-endian byte order][ex-byteorder-le] | [![byteorder-badge]][byteorder] | [![cat-encoding-badge]][cat-encoding] |
| [Generate random floating point numbers][ex-rand-float] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random numbers within a range][ex-rand-range] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random numbers with normal distribution][ex-rand-dist] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random values of a custom type][ex-rand-custom] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |


[ex-std-read-lines]: #ex-std-read-lines
<a name="ex-std-read-lines"></a>
## Read lines of strings from a file

[![std-badge]][std] [![cat-filesystem-badge]][cat-filesystem]

Writes a three-line message to a file, then reads it back a line at a
time with the [`Lines`] iterator created by
[`BufRead::lines`]. [`BufRead`] is a trait, and the most common way to
get one is from a [`BufReader`], which is constructed from some type
that implements [`Write`], here a [`File`]. The [`File`] is opened
for writing with [`File::create`], and reading with [`File::open`].

```rust
#[macro_use]
extern crate error_chain;

use std::fs::File;
use std::io::{Write, BufReader, BufRead};

error_chain! {
    foreign_links {
        Io(std::io::Error);
    }
}

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

quick_main!(run);
```

[ex-byteorder-le]: #ex-byteorder-le
<a name="ex-byteorder-le"></a>
## Read and write integers in little-endian byte order

[![byteorder-badge]][byteorder] [![cat-encoding-badge]][cat-encoding]

```rust
extern crate byteorder;

#[macro_use]
extern crate error_chain;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[derive(Default, PartialEq, Debug)]
struct Payload {
    kind: u8,
    value: u16,
}

error_chain! {
    foreign_links {
        Io(std::io::Error);
    }
}

fn run() -> Result<()> {
    let original_payload = Payload::default();
    let encoded_bytes = encode(&original_payload)?;
    let decoded_payload = decode(&encoded_bytes)?;
    assert_eq!(original_payload, decoded_payload);
    Ok(())
}

fn encode(payload: &Payload) -> Result<Vec<u8>> {
    let mut bytes = vec![];
    bytes.write_u8(payload.kind)?;
    bytes.write_u16::<LittleEndian>(payload.value)?;
    Ok(bytes)
}

fn decode(mut bytes: &[u8]) -> Result<Payload> {
    let payload = Payload {
        kind: bytes.read_u8()?,
        value: bytes.read_u16::<LittleEndian>()?,
    };
    Ok(payload)
}

quick_main!(run);
```

[ex-rand-float]: #ex-rand-float
<a name="ex-rand-float"></a>
## Generate random floating point numbers

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

```rust
extern crate rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Random f64: {}", rng.gen::<f64>());
}
```

[ex-rand-range]: #ex-rand-range
<a name="ex-rand-range"></a>
## Generate random numbers within a range

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

A random value within a range `[0, 10)` (not including `10`) is generated with [`Rng::gen_range`].

```rust
extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("{}", rng.gen_range(0, 10));
}
```

[ex-rand-dist]: #ex-rand-dist
<a name="ex-rand-dist"></a>
## Generate random numbers with normal distribution

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

[`Normal`] distribution with mean `3` and standard deviation `5`
is created. A random value is generated with [`IndependentSample::ind_sample`].

```rust
extern crate rand;

use rand::distributions::{Normal, IndependentSample};

fn main() {
    let normal = Normal::new(3.0, 5.0);
    let mut rng = rand::thread_rng();
    let v = normal.ind_sample(&mut rng);
    println!("{} is from a N(3, 25) distribution", v)
}
```

[ex-rand-custom]: #ex-rand-custom
<a name="ex-rand-custom"></a>
## Generate random values of a custom type

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

A tuple `(i32, bool, f64)` and variable of user defined type `Point`
are randomly generated. In order to allow random generation of `Point`
it needs to implement the [`rand::Rand`] trait.

```rust
extern crate rand;
use rand::{Rng, Rand};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Rand for Point {
    fn rand<R: Rng>(rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point { x: rand_x, y: rand_y }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}
```

<!-- Categories -->

[cat-encoding-badge]: https://img.shields.io/badge/-encoding-red.svg
[cat-encoding]: https://crates.io/categories/encoding
[cat-filesystem-badge]: https://img.shields.io/badge/-filesystem-red.svg
[cat-filesystem]: https://crates.io/categories/filesystem
[cat-science-badge]: https://img.shields.io/badge/-rand-red.svg
[cat-science]: https://crates.io/categories/science

<!-- Crates -->

[byteorder-badge]: https://img.shields.io/crates/v/byteorder.svg?label=byteorder
[byteorder]: https://docs.rs/byteorder/
[rand-badge]: https://img.shields.io/crates/v/rand.svg?label=rand
[rand]: https://docs.rs/rand/
[std-badge]: https://img.shields.io/badge/std-1.17.0-blue.svg
[std]: https://doc.rust-lang.org/std

<!-- API links -->

[`BufRead::lines`]: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
[`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html
[`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`File::create`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.create
[`File::open`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.open
[`Lines`]: https://doc.rust-lang.org/std/io/struct.Lines.html
[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
[`Normal`]: https://doc.rust-lang.org/rand/rand/distributions/normal/struct.Normal.html
[`IndependentSample::ind_sample`]: https://doc.rust-lang.org/rand/rand/distributions/trait.IndependentSample.html#tymethod.ind_sample
[`Rng::gen_range`]: https://doc.rust-lang.org/rand/rand/trait.Rng.html#method.gen_range
[`rand::Rand`]: https://doc.rust-lang.org/rand/rand/trait.Rand.html
