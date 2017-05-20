# Basics

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Read lines of strings from a file][ex-std-read-lines] | [![std-badge]][std] | [![cat-filesystem-badge]][cat-filesystem] |
| [Read and write integers in little-endian byte order][ex-byteorder-le] | [![byteorder-badge]][byteorder] | [![cat-encoding-badge]][cat-encoding] |
| [Generate random floating point numbers][ex-rand-float] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random numbers within a range][ex-rand-range] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random numbers with normal distribution][ex-rand-dist] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random values of a custom type][ex-rand-custom] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Run an External Command and Process Stdout][ex-parse-subprocess-output] | [![regex-badge]][regex] | [![cat-os-badge]][cat-os] [![cat-text-processing-badge]][cat-text-processing] |
| [Declare lazily evaluated constant][ex-lazy-constant] | [![lazy_static-badge]][lazy_static] | [![cat-caching-badge]][cat-caching] [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Maintain global mutable state][ex-global-mut-state] | [![lazy_static-badge]][lazy_static] | [![cat-rust-patterns-badge]][cat-rust-patterns] |



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
# #[macro_use]
# extern crate error_chain;
#
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
#
# error_chain! {
#    foreign_links {
#        Io(std::io::Error);
#    }
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

[ex-byteorder-le]: #ex-byteorder-le
<a name="ex-byteorder-le"></a>
## Read and write integers in little-endian byte order

[![byteorder-badge]][byteorder] [![cat-encoding-badge]][cat-encoding]

```rust
# #[macro_use]
# extern crate error_chain;
extern crate byteorder;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[derive(Default, PartialEq, Debug)]
struct Payload {
    kind: u8,
    value: u16,
}
#
# error_chain! {
#    foreign_links {
#        Io(std::io::Error);
#    }
# }

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
#
# quick_main!(run);
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

[ex-parse-subprocess-output]: #ex-parse-subprocess-output
<a name="ex-parse-subprocess-output"></a>
## Run an External Command and Process Stdout

[![regex-badge]][regex] [![cat-os-badge]][cat-os] [![cat-text-processing-badge]][cat-text-processing]

`git log --oneline` is run as an external [`Command`] and its [`Output`] is
inspected using [`Regex`] to get the hash and message of the last 5 commits.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate regex;

use std::process::Command;
use regex::Regex;
#
# error_chain!{
#    foreign_links {
#        Io(std::io::Error);
#        Regex(regex::Error);
#        Utf8(std::string::FromUtf8Error);
#    }
# }

#[derive(PartialEq, Default, Clone, Debug)]
struct Commit {
    hash: String,
    message: String,
}

fn run() -> Result<()> {
    let output = Command::new("git").arg("log").arg("--oneline").output()?;

    if !output.status.success() {
        bail!("Command executed with failing error code");
    }

    let pattern = Regex::new(r"(?x)
                               ([0-9a-fA-F]+) # commit hash
                               (.*)           # The commit message")?;

    let stdout = String::from_utf8(output.stdout)?;
    let commits = stdout
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| {
                 Commit {
                     hash: cap[1].to_string(),
                     message: cap[2].trim().to_string(),
                 }
             })
        .take(5);

    for commit in commits {
        println!("{:?}", commit);
    }

    Ok(())
}
#
# quick_main!(run);
```

[ex-lazy-constant]: #ex-lazy-constant
<a name="ex-lazy-constant"></a>
## Declare lazily evaluated constant

[![lazy_static-badge]][lazy_static] [![cat-caching-badge]][cat-caching] [![cat-rust-patterns-badge]][cat-rust-patterns]

Declares a lazily evaluated constant [`HashMap`]. The [`HashMap`] will
be evaluated once and stored behind a global static reference.

```rust
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref PRIVILEGES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("James", vec!["user", "admin"]);
        map.insert("Jim", vec!["user"]);
        map
    };
}

fn show_access(name: &str) {
    let access = PRIVILEGES.get(name);
    println!("{}: {:?}", name, access);
}

fn main() {
    let access = PRIVILEGES.get("James");
    println!("James: {:?}", access);

    show_access("Jim");
}
```

[ex-global-mut-state]: #ex-global-mut-state
<a name="ex-global-mut-state"></a>
## Maintain global mutable state

[![lazy_static-badge]][lazy_static] [![cat-rust-patterns-badge]][cat-rust-patterns]

Declares some global state using [lazy_static]. Since [lazy_static]
creates a globally available `static ref` we also need to wrap our state
in a [`Mutex`] to allow mutation (also see [`RwLock`]). The [`Mutex`] ensures
the state cannot be simultaneously accessed by multiple threads, preventing
race conditions. A [`MutexGuard`] must be acquired to read or mutate the
value stored in a [`Mutex`].

```rust
# #[macro_use]
# extern crate error_chain;
#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
#
# error_chain!{ }

lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn insert(fruit: &str) -> Result<()> {
    // acquire exclusive access
    let mut db = FRUIT.lock()
        .map_err(|_| "Failed to acquire MutexGuard")?;
    db.push(fruit.to_string());
    Ok(())
    // release exclusive access
}

fn run() -> Result<()> {
    insert("apple")?;
    insert("orange")?;
    insert("peach")?;
    {
        // acquire access
        let db = FRUIT.lock()
            .map_err(|_| "Failed to acquire MutexGuard")?;

        for (i, item) in db.iter().enumerate() {
            println!("{}: {}", i, item);
        }
        // release access
    }
    insert("grape")?;
    Ok(())
}
#
# quick_main!(run);
```

<!-- Categories -->

[cat-caching-badge]: https://img.shields.io/badge/-caching-red.svg
[cat-caching]: https://crates.io/categories/caching
[cat-encoding-badge]: https://img.shields.io/badge/-encoding-red.svg
[cat-encoding]: https://crates.io/categories/encoding
[cat-filesystem-badge]: https://img.shields.io/badge/-filesystem-red.svg
[cat-filesystem]: https://crates.io/categories/filesystem
[cat-science-badge]: https://img.shields.io/badge/-rand-red.svg
[cat-science]: https://crates.io/categories/science
[cat-os-badge]: https://img.shields.io/badge/-os-red.svg
[cat-os]: https://crates.io/categories/os
[cat-rust-patterns-badge]: https://img.shields.io/badge/-rust_patterns-red.svg
[cat-rust-patterns]: https://crates.io/categories/rust-patterns
[cat-text-processing-badge]: https://img.shields.io/badge/-text_processing-red.svg
[cat-text-processing]: https://crates.io/categories/text-processing

<!-- Crates -->

[byteorder-badge]: https://img.shields.io/crates/v/byteorder.svg?label=byteorder
[byteorder]: https://docs.rs/byteorder/
[lazy_static]: https://docs.rs/lazy_static/
[lazy_static-badge]: https://img.shields.io/crates/v/lazy_static.svg?label=lazy_static
[rand-badge]: https://img.shields.io/crates/v/rand.svg?label=rand
[rand]: https://docs.rs/rand/
[std-badge]: https://img.shields.io/badge/std-1.17.0-blue.svg
[std]: https://doc.rust-lang.org/std
[regex]: https://docs.rs/regex/
[regex-badge]: https://img.shields.io/crates/v/regex.svg?label=regex

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
[`Regex`]: https://doc.rust-lang.org/regex/regex/struct.Regex.html
[`Output`]: https://doc.rust-lang.org/std/process/struct.Output.html
[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[`RwLock`]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
[`MutexGuard`]: https://doc.rust-lang.org/std/sync/struct.MutexGuard.html
