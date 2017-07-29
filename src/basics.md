# Basics

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Read lines of strings from a file][ex-std-read-lines] | [![std-badge]][std] | [![cat-filesystem-badge]][cat-filesystem] |
| [Read and write integers in little-endian byte order][ex-byteorder-le] | [![byteorder-badge]][byteorder] | [![cat-encoding-badge]][cat-encoding] |
| [Generate random numbers][ex-rand] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random numbers within a range][ex-rand-range] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random distributions][ex-rand-dist] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random values of a custom type][ex-rand-custom] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Run an external command and process stdout][ex-parse-subprocess-output] | [![regex-badge]][regex] | [![cat-os-badge]][cat-os] [![cat-text-processing-badge]][cat-text-processing] |
| [Filter a log file by matching multiple regular expressions][ex-regex-filter-log] | [![regex-badge]][regex] | [![cat-text-processing-badge]][cat-text-processing]
| [Declare lazily evaluated constant][ex-lazy-constant] | [![lazy_static-badge]][lazy_static] | [![cat-caching-badge]][cat-caching] [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Maintain global mutable state][ex-global-mut-state] | [![lazy_static-badge]][lazy_static] | [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Access a file randomly using a memory map][ex-random-file-access] | [![memmap-badge]][memmap] | [![cat-filesystem-badge]][cat-filesystem] |
| [Define and operate on a type represented as a bitfield][ex-bitflags] | [![bitflags-badge]][bitflags] | [![cat-no-std-badge]][cat-no-std] |
| [Extract a list of unique #Hashtags from a text][ex-extract-hashtags] | [![regex-badge]][regex] [![lazy_static-badge]][lazy_static] | [![cat-text-processing-badge]][cat-text-processing] |
| [Replace all occurrences of one text pattern with another pattern.][ex-regex-replace-named] | [![regex-badge]][regex] [![lazy_static-badge]][lazy_static] | [![cat-text-processing-badge]][cat-text-processing] |
| [Extract phone numbers from text][ex-phone] | [![regex-badge]][regex] | [![cat-text-processing-badge]][cat-text-processing] |
| [Calculate the SHA-256 digest of a file][ex-sha-digest] | [![ring-badge]][ring] [![data-encoding-badge]][data-encoding] | [![cat-cryptography-badge]][cat-cryptography] |


[ex-std-read-lines]: #ex-std-read-lines
<a name="ex-std-read-lines"></a>
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
#     foreign_links {
#         Io(std::io::Error);
#     }
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

[ex-rand]: #ex-rand
<a name="ex-rand"></a>
## Generate random numbers

The [`rand`] crate provides a convenient source of psuedo-random numbers.

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

```rust
extern crate rand;
use rand::Rng;

fn main() {
    // Each thread has an automatically-initialised random number generator:
    let mut rng = rand::thread_rng();
    
    // Integers are uniformly distributed over the type's whole range:
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}; random u16: {}", n1, n2);
    println!("Random i32: {}", rng.gen::<i32>());
    
    // Floating point numbers are uniformly distributed in the half-open range [0, 1)
    println!("Random float: {}", rng.gen::<f64>());
}
```

[ex-rand-range]: #ex-rand-range
<a name="ex-rand-range"></a>
## Generate random numbers within a range

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

Generates a random value within `[0, 10)` range (not including `10`) with [`Rng::gen_range`].

```rust
extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0, 10));
    println!("Float: {}", rng.gen_range(0.0, 10.0));
}
```

Alternatively, one can use the [`Range`] distribution (otherwise known as uniform).
This has the same effect, but may be faster when repeatedly generating numbers
in the same range.

```rust
extern crate rand;

use rand::distributions::{Range, IndependentSample};

fn main() {
    let mut rng = rand::thread_rng();
    let die = Range::new(1, 7);
    
    loop {
        let throw = die.ind_sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}
```


[ex-rand-dist]: #ex-rand-dist
<a name="ex-rand-dist"></a>
## Random number distributions

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

By default, random numbers are generated with a uniform distribution.
To generate numbers with other distributions you create a
distribution, then sample from that distribution (using
[`IndependentSample::ind_sample`]) with the help of a random-number
generator (`rng`).

The [distributions available are documented here][rand-distributions]. An example using the
[`Normal`] distribution is shown below.

```rust
extern crate rand;

use rand::distributions::{Normal, IndependentSample};

fn main() {
    let mut rng = rand::thread_rng();
    
    // mean 2, standard deviation 3:
    let normal = Normal::new(2.0, 3.0);
    let v = normal.ind_sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v)
}
```

[ex-rand-custom]: #ex-rand-custom
<a name="ex-rand-custom"></a>
## Generate random values of a custom type

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

Randomly generates a tuple `(i32, bool, f64)` and variable of user defined type `Point`.
Implements the [`rand::Rand`] trait for `Point` in order to allow random generation.

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
        Point {
            x: rand_x,
            y: rand_y,
        }
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
## Run an external command and process stdout

[![regex-badge]][regex] [![cat-os-badge]][cat-os] [![cat-text-processing-badge]][cat-text-processing]

Runs `git log --oneline` as an external [`Command`] and inspects its [`Output`]
using [`Regex`] to get the hash and message of the last 5 commits.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate regex;

use std::process::Command;
use regex::Regex;
#
# error_chain!{
#     foreign_links {
#         Io(std::io::Error);
#         Regex(regex::Error);
#         Utf8(std::string::FromUtf8Error);
#     }
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

[ex-regex-filter-log]: #ex-regex-filter-log
<a name="ex-regex-filter-log"></a>
## Filter a log file by matching multiple regular expressions

[![regex-badge]][regex] [![cat-text-processing-badge]][cat-text-processing]

Reads a file named `application.log` and only outputs the lines 
containing “version X.X.X”, some IP address followed by port 443
(e.g. “192.168.0.1:443”), or a specific warning.

A [`regex::RegexSet`] is built with [`regex::RegexSetBuilder`].
Since backslashes are very common in regular expressions, using
[raw string literals] make them more readable.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate regex;

use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::RegexSetBuilder;

# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#         Regex(regex::Error);
#     }
# }
#
fn run() -> Result<()> {
    let log_path = "application.log";
    let buffered = BufReader::new(File::open(log_path)?);

    let set = RegexSetBuilder::new(&[
        r#"version "\d\.\d\.\d""#,
        r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:443"#,
        r#"warning.*timeout expired"#,
    ]).case_insensitive(true)
        .build()?;

    let filtered = buffered
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| set.is_match(line.as_str()));

    for line in filtered {
        println!("{}", line);
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
    let mut db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
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
        let db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;

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

[ex-random-file-access]: #ex-random-file-access
<a name="ex-random-file-access"></a>
## Access a file randomly using a memory map

[![memmap-badge]][memmap] [![cat-filesystem-badge]][cat-filesystem]

Creates a memory map of a file using [memmap] and simulates some non-sequential
reads from the file. Using a memory map means you just index into a slice rather
than dealing with [`seek`]ing around in a File.

The [`Mmap::as_slice`] function is only safe if we can guarantee that the file
behind the memory map is not being modified at the same time by another process,
as this would be a [race condition][race-condition-file].

```rust
# #[macro_use]
# extern crate error_chain;
extern crate memmap;

use memmap::{Mmap, Protection};
# use std::fs::File;
# use std::io::Write;
#
# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#     }
# }

fn run() -> Result<()> {
#     write!(File::create("content.txt")?, "My hovercraft is full of eels!")?;
#
    let map = Mmap::open_path("content.txt", Protection::Read)?;
    let random_indexes = [0, 1, 2, 19, 22, 10, 11, 29];
    // This is only safe if no other code is modifying the file at the same time
    unsafe {
        let map = map.as_slice();
        assert_eq!(&map[3..13], b"hovercraft");
        // I'm using an iterator here to change indexes to bytes
        let random_bytes: Vec<u8> = random_indexes.iter()
            .map(|&idx| map[idx])
            .collect();
        assert_eq!(&random_bytes[..], b"My loaf!");
    }
    Ok(())
}
#
# quick_main!(run);
```

[ex-bitflags]: #ex-bitflags
<a name="ex-bitflags"></a>
## Define and operate on a type represented as a bitfield

[![bitflags-badge]][bitflags] [![cat-no-std-badge]][cat-no-std]

Creates typesafe bitfield type `MyFlags` with help of [`bitflags!`] macro
and implements elementary `clear` operation as well as [`Display`] trait for it.
Subsequently, shows basic bitwise operations and formatting.

```rust
#[macro_use]
extern crate bitflags;

use std::fmt;

bitflags! {
    struct MyFlags: u32 {
        const FLAG_A       = 0b00000001;
        const FLAG_B       = 0b00000010;
        const FLAG_C       = 0b00000100;
        const FLAG_ABC     = FLAG_A.bits
                           | FLAG_B.bits
                           | FLAG_C.bits;
    }
}

impl MyFlags {
    pub fn clear(&mut self) -> &mut MyFlags {
        self.bits = 0;  // The `bits` field can be accessed from within the
                        // same module where the `bitflags!` macro was invoked.
        self
    }
}

impl fmt::Display for MyFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:032b}", self.bits)
    }
}

fn main() {
    let e1 = FLAG_A | FLAG_C;
    let e2 = FLAG_B | FLAG_C;
    assert_eq!((e1 | e2), FLAG_ABC);   // union
    assert_eq!((e1 & e2), FLAG_C);     // intersection
    assert_eq!((e1 - e2), FLAG_A);     // set difference
    assert_eq!(!e2, FLAG_A);           // set complement

    let mut flags = FLAG_ABC;
    assert_eq!(format!("{}", flags), "00000000000000000000000000000111");
    assert_eq!(format!("{}", flags.clear()), "00000000000000000000000000000000");
    // Debug trait is automatically derived for the MyFlags through `bitflags!`
    assert_eq!(format!("{:?}", FLAG_B), "FLAG_B");
    assert_eq!(format!("{:?}", FLAG_A | FLAG_B), "FLAG_A | FLAG_B");
}
```

[ex-extract-hashtags]: #ex-extract-hashtags
<a name="ex-extract-hashtags"></a>
## Extract a list of unique #Hashtags from a text

[![regex-badge]][regex] [![lazy_static-badge]][lazy_static] [![cat-text-processing-badge]][cat-text-processing]

Extracts a sorted and deduplicated list of hashtags from a text.

The hashtag regex given here only catches latin hashtags that start with a letter. The complete [twitter hashtag regex] is way more complicated.

```rust
extern crate regex;
#[macro_use] extern crate lazy_static;

use regex::Regex;
use std::collections::HashSet;

/// Note: A HashSet does not contain duplicate values.
fn extract_hashtags(text: &str) -> HashSet<&str> {
    lazy_static! {
        static ref HASHTAG_REGEX : Regex = Regex::new(
                r"\#[a-zA-Z][0-9a-zA-Z_]*"
            ).unwrap();
    }
    HASHTAG_REGEX.find_iter(text).map(|mat| mat.as_str()).collect()
}

fn main() {
    let tweet = "Hey #world, I just got my new #dog, say hello to Till. #dog #forever #2 #_ ";
    let tags = extract_hashtags(tweet);
    assert!(tags.contains("#dog") && tags.contains("#forever") && tags.contains("#world"));
    assert_eq!(tags.len(), 3);
}
```

[ex-regex-replace-named]: #ex-regex-replace-named
<a name="ex-regex-replace-named"></a>
## Replace all occurrences of one text pattern with another pattern.

[![regex-badge]][regex] [![lazy_static-badge]][lazy_static] [![cat-text-processing-badge]][cat-text-processing]

Replaces all occurences of the hyphenated british english date pattern `2013-01-15` 
with its equivalent slashed american english date pattern `01/15/2013`.

The method [`Regex::replace_all`] replaces all occurrences of the whole regex. The
`Replacer` trait helps to figure out the replacement string. This trait is implemented
for `&str` and allows to use variables like `$abcde` to refer to corresponding named capture groups
`(?P<abcde>REGEX)` from the search regex. See the [replacement string syntax] for examples
and information about escaping.

```rust
extern crate regex;
#[macro_use] extern crate lazy_static;

use std::borrow::Cow;
use regex::Regex;

fn reformat_dates(before: &str) -> Cow<str> {
    lazy_static! {
        static ref ENGL_DATE_REGEX : Regex = Regex::new(
            r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})"
            ).unwrap();
    }
    ENGL_DATE_REGEX.replace_all(before, "$m/$d/$y")
}

fn main() {
    let before = "2012-03-14, 2013-01-01 and 2014-07-05";
    let after = reformat_dates(before);
    assert_eq!(after, "03/14/2012, 01/01/2013 and 07/05/2014");
}
```

[ex-phone]: #ex-phone
<a name="ex-phone"></a>
## Extract phone numbers from text

Processes a string of text using [`Regex::captures_iter`] to capture multiple
phone numbers.  The example here is for US convention phone numbers.

```rust, no_run
# #[macro_use]
# extern crate error_chain;
extern crate regex;

use regex::Regex;
use std::fmt;

# error_chain!{
#     foreign_links {
#         Regex(regex::Error);
#         Io(std::io::Error);
#     }
# }
#
#[derive(PartialEq, PartialOrd, Debug)]
struct PhoneNumber {
    area: &'static str,
    exchange: &'static str,
    subscriber: &'static str,
}

// Allows printing phone numbers based on country convention.
impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "1 ({}) {}-{}",
            &self.area,
            &self.exchange,
            &self.subscriber
        )
    }
}

fn run() -> Result<()> {
    let phone_text = "
    +1 505 881 9292 (v) +1 505 778 2212 (c) +1 505 881 9297 (f)
    (202) 991 9534
    Alex 5553920011
    1 (800) 233-2010
    1.299.339.1020";
    let re = Regex::new(
        r"(?x)
                        (?:\+?1)?                         # Country Code Optional
                        [\s\.]?
                        (([2-9]\d{2})|\(([2-9]\d{2})\))  # Area Code
                        [\s\.\-]?
                        ([2-9]\d{2})                     # Exchange Code
                        [\s\.\-]?
                        (\d{4})                          #Subscriber Number",
    )?;
    let mut phone_numbers = re.captures_iter(phone_text).map(|cap| {
        // Area code populates either capture group 2 or 3.  Group 1 contains optional paranthesis.
        PhoneNumber {
            area: if cap.get(2) == None {
                cap.get(3).map_or("", |m| m.as_str())
            } else {
                cap.get(2).map_or("", |m| m.as_str())
            },
            exchange: cap.get(4).map_or("", |m| m.as_str()),
            subscriber: cap.get(5).map_or("", |m| m.as_str()),
        }
    });    assert_eq!(
        phone_numbers.next().map(|m| m.to_string()),
        Some("1 (505) 881-9292".to_owned())
    );
    assert_eq!(
        phone_numbers.next().map(|m| m.to_string()),
        Some("1 (505) 778-2212".to_owned())
    );
    assert_eq!(
        phone_numbers.next().map(|m| m.to_string()),
        Some("1 (505) 881-9297".to_owned())
    );
    assert_eq!(
        phone_numbers.next().map(|m| m.to_string()),
        Some("1 (202) 991-9534".to_owned())
    );
    assert_eq!(
        phone_numbers.next().map(|m| m.to_string()),
        Some("1 (555) 392-0011".to_owned())
    );
    assert_eq!(
        phone_numbers.next().map(|m| m.to_string()),
        Some("1 (800) 233-2010".to_owned())
    );
    Ok(())
}
#
# quick_main!(run);

```

[ex-sha-digest]: #ex-sha-digest
<a name="ex-sha-digest"></a>
## Calculate the SHA-256 digest of a file

[![ring-badge]][ring] [![data-encoding-badge]][data-encoding] [![cat-cryptography-badge]][cat-cryptography]

Writes some data to a file, then calculates the SHA-256 [`digest::Digest`] of
the file's contents using [`digest::Context`].

```rust
# #[macro_use]
# extern crate error_chain;
extern crate data_encoding;
extern crate ring;

use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read, Write};
#
# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#         Decode(data_encoding::DecodeError);
#     }
# }

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    loop {
        let mut buffer = [0; 1024];
        let count = reader.read(&mut buffer[..])?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

fn run() -> Result<()> {
    let path = "file.txt";

    let mut output = File::create(path)?;
    write!(output, "We will generate a digest of this text")?;

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;

    // digest.as_ref() provides the digest as a byte slice: &[u8]
    println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));

    Ok(())
}
#
# quick_main!(run);
```

<!-- Categories -->

[cat-no-std-badge]: https://badge-cache.kominick.com/badge/no_std--x.svg?style=social
[cat-no-std]: https://crates.io/categories/no-std
[cat-caching-badge]: https://badge-cache.kominick.com/badge/caching--x.svg?style=social
[cat-caching]: https://crates.io/categories/caching
[cat-cryptography-badge]: https://badge-cache.kominick.com/badge/cryptography--x.svg?style=social
[cat-cryptography]: https://crates.io/categories/cryptography
[cat-encoding-badge]: https://badge-cache.kominick.com/badge/encoding--x.svg?style=social
[cat-encoding]: https://crates.io/categories/encoding
[cat-filesystem-badge]: https://badge-cache.kominick.com/badge/filesystem--x.svg?style=social
[cat-filesystem]: https://crates.io/categories/filesystem
[cat-science-badge]: https://badge-cache.kominick.com/badge/science--x.svg?style=social
[cat-science]: https://crates.io/categories/science
[cat-os-badge]: https://badge-cache.kominick.com/badge/OS--x.svg?style=social
[cat-os]: https://crates.io/categories/os
[cat-rust-patterns-badge]: https://badge-cache.kominick.com/badge/rust_patterns--x.svg?style=social
[cat-rust-patterns]: https://crates.io/categories/rust-patterns
[cat-text-processing-badge]: https://badge-cache.kominick.com/badge/text_processing--x.svg?style=social
[cat-text-processing]: https://crates.io/categories/text-processing

<!-- Crates -->

[bitflags-badge]: https://badge-cache.kominick.com/crates/v/bitflags.svg?label=bitflags
[bitflags]: https://docs.rs/bitflags/
[byteorder-badge]: https://badge-cache.kominick.com/crates/v/byteorder.svg?label=byteorder
[byteorder]: https://docs.rs/byteorder/
[data-encoding-badge]: https://badge-cache.kominick.com/crates/v/data-encoding.svg?label=data-encoding
[data-encoding]: https://docs.rs/data-encoding/
[lazy_static]: https://docs.rs/lazy_static/
[lazy_static-badge]: https://badge-cache.kominick.com/crates/v/lazy_static.svg?label=lazy_static
[rand-badge]: https://badge-cache.kominick.com/crates/v/rand.svg?label=rand
[rand]: https://docs.rs/rand/
[std-badge]: https://badge-cache.kominick.com/badge/std-1.17.0-blue.svg
[std]: https://doc.rust-lang.org/std
[regex]: https://docs.rs/regex/
[regex-badge]: https://badge-cache.kominick.com/crates/v/regex.svg?label=regex
[memmap]: https://docs.rs/memmap/
[memmap-badge]: https://badge-cache.kominick.com/crates/v/memmap.svg?label=memmap
[ring]: https://docs.rs/ring/
[ring-badge]: https://badge-cache.kominick.com/crates/v/ring.svg?label=ring

<!-- API links -->

[`bitflags!`]: https://docs.rs/bitflags/*/bitflags/macro.bitflags.html
[`BufRead::lines`]: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
[`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html
[`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`File::create`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.create
[`File::open`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.open
[`Lines`]: https://doc.rust-lang.org/std/io/struct.Lines.html
[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`Normal`]: https://doc.rust-lang.org/rand/rand/distributions/normal/struct.Normal.html
[`IndependentSample::ind_sample`]: https://doc.rust-lang.org/rand/rand/distributions/trait.IndependentSample.html#tymethod.ind_sample
[`rand`]: https://doc.rust-lang.org/rand
[`Rng::gen_range`]: https://doc.rust-lang.org/rand/rand/trait.Rng.html#method.gen_range
[`Regex::captures_iter`]: https://doc.rust-lang.org/regex/regex/struct.Regex.html#method.captures_iter
[`Regex::replace_all`]: https://docs.rs/regex/0.2.2/regex/struct.Regex.html#method.replace_all
[`rand::Rand`]: https://doc.rust-lang.org/rand/rand/trait.Rand.html
[`Range`]: https://doc.rust-lang.org/rand/rand/distributions/range/struct.Range.html
[rand-distributions]: https://doc.rust-lang.org/rand/rand/distributions/index.html
[`Regex`]: https://doc.rust-lang.org/regex/regex/struct.Regex.html
[`rand::Rand`]: https://doc.rust-lang.org/rand/rand/trait.Rand.html
[`regex::RegexSetBuilder`]: https://doc.rust-lang.org/regex/regex/struct.RegexSetBuilder.html
[`regex::RegexSet`]: https://doc.rust-lang.org/regex/regex/struct.RegexSet.html
[replacement string syntax]: https://docs.rs/regex/0.2.2/regex/struct.Regex.html#replacement-string-syntax
[`Output`]: https://doc.rust-lang.org/std/process/struct.Output.html
[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[`RwLock`]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
[`MutexGuard`]: https://doc.rust-lang.org/std/sync/struct.MutexGuard.html
[`Mmap::as_slice`]: https://docs.rs/memmap/*/memmap/struct.Mmap.html#method.as_slice
[`seek`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.seek
[`digest::Context`]: https://docs.rs/ring/*/ring/digest/struct.Context.html
[`digest::Digest`]: https://docs.rs/ring/*/ring/digest/struct.Digest.html

<!-- Reference -->

[race-condition-file]: https://en.wikipedia.org/wiki/Race_condition#File_systems
[raw string literals]: https://doc.rust-lang.org/reference/tokens.html#raw-string-literals
[twitter hashtag regex]: https://github.com/twitter/twitter-text/blob/master/java/src/com/twitter/Regex.java#L255
