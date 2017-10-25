# Basics

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Read lines of strings from a file][ex-std-read-lines] | [![std-badge]][std] | [![cat-filesystem-badge]][cat-filesystem] |
| [Read and write integers in little-endian byte order][ex-byteorder-le] | [![byteorder-badge]][byteorder] | [![cat-encoding-badge]][cat-encoding] |
| [Generate random numbers][ex-rand] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random numbers within a range][ex-rand-range] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random numbers with given distribution][ex-rand-dist] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random values of a custom type][ex-rand-custom] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Run an external command and process stdout][ex-parse-subprocess-output] | [![regex-badge]][regex] | [![cat-os-badge]][cat-os] [![cat-text-processing-badge]][cat-text-processing] |
| [Run an external command passing it stdin and check for an error code][ex-parse-subprocess-input] | [![regex-badge]][regex] | [![cat-os-badge]][cat-os] [![cat-text-processing-badge]][cat-text-processing] |
| [Run piped external commands][ex-run-piped-external-commands] | [![std-badge]][std] | [![cat-os-badge]][cat-os] |
| [Filter a log file by matching multiple regular expressions][ex-regex-filter-log] | [![regex-badge]][regex] | [![cat-text-processing-badge]][cat-text-processing]
| [Declare lazily evaluated constant][ex-lazy-constant] | [![lazy_static-badge]][lazy_static] | [![cat-caching-badge]][cat-caching] [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Maintain global mutable state][ex-global-mut-state] | [![lazy_static-badge]][lazy_static] | [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Verify and extract login from an email address][ex-verify-extract-email] | [![regex-badge]][regex] [![lazy_static-badge]][lazy_static] | [![cat-text-processing-badge]][cat-text-processing] |
| [Extract a list of unique #Hashtags from a text][ex-extract-hashtags] | [![regex-badge]][regex] [![lazy_static-badge]][lazy_static] | [![cat-text-processing-badge]][cat-text-processing] |
| [Replace all occurrences of one text pattern with another pattern.][ex-regex-replace-named] | [![regex-badge]][regex] [![lazy_static-badge]][lazy_static] | [![cat-text-processing-badge]][cat-text-processing] |
| [Extract phone numbers from text][ex-phone] | [![regex-badge]][regex] | [![cat-text-processing-badge]][cat-text-processing] |
| [Calculate the SHA-256 digest of a file][ex-sha-digest] | [![ring-badge]][ring] [![data-encoding-badge]][data-encoding] | [![cat-cryptography-badge]][cat-cryptography] |
| [Sign and verify a message with an HMAC digest][ex-hmac] | [![ring-badge]][ring] | [![cat-cryptography-badge]][cat-cryptography] |
| [Salt and hash a password with PBKDF2][ex-pbkdf2] | [![ring-badge]][ring] [![data-encoding-badge]][data-encoding] | [![cat-cryptography-badge]][cat-cryptography] |
| [Define and operate on a type represented as a bitfield][ex-bitflags] | [![bitflags-badge]][bitflags] | [![cat-no-std-badge]][cat-no-std] |
| [Access a file randomly using a memory map][ex-random-file-access] | [![memmap-badge]][memmap] | [![cat-filesystem-badge]][cat-filesystem] |
| [Check number of logical cpu cores][ex-check-cpu-cores] | [![num_cpus-badge]][num_cpus] | [![cat-hardware-support-badge]][cat-hardware-support] |
| [Obtain backtrace of complex error scenarios][ex-error-chain-backtrace] | [![error-chain-badge]][error-chain] | [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Measure elapsed time][ex-measure-elapsed-time] | [![std-badge]][std] | [![cat-time-badge]][cat-time] |
| [Display formatted date and time][ex-format-datetime] | [![chrono-badge]][chrono] | [![cat-date-and-time-badge]][cat-date-and-time] |

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

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

Generates random numbers with help of random-number
generator [`rand::Rng`] obtained via [`rand::thread_rng`].

```rust
extern crate rand;

use rand::Rng;

fn main() {
    // Each thread has an automatically-initialised random number generator:
    let mut rng = rand::thread_rng();

    // Integers are uniformly distributed over the type's whole range:
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());

    // Floating point numbers are uniformly distributed in the half-open range [0, 1)
    println!("Random float: {}", rng.gen::<f64>());
}
```

[ex-rand-range]: #ex-rand-range
<a name="ex-rand-range"></a>
## Generate random numbers within a range

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

Generates a random value within half-open `[0, 10)` range (not including `10`) with [`Rng::gen_range`].

```rust
extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0, 10));
    println!("Float: {}", rng.gen_range(0.0, 10.0));
}
```

Alternatively, one can use [`Range`] to obtain values with [uniform distribution].
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

## Generate random numbers with given distribution

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

By default, random numbers are generated with [uniform distribution].
To generate numbers with other distributions you instantiate a
distribution, then sample from that distribution using
[`IndependentSample::ind_sample`] with help of a random-number
generator [`rand::Rng`].

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

    String::from_utf8(output.stdout)?
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| {
                 Commit {
                     hash: cap[1].to_string(),
                     message: cap[2].trim().to_string(),
                 }
             })
        .take(5)
        .for_each(|x| println!("{:?}", x));

    Ok(())
}
#
# quick_main!(run);
```

[ex-parse-subprocess-input]: #ex-parse-subprocess-input
<a name="ex-parse-subprocess-input"></a>
## Run an external command passing it stdin and check for an error code

[![std-badge]][std] [![cat-os-badge]][cat-os]

Opens the `python` interpreter using an external [`Command`] and passes it a python statement
for execution. [`Output`] of said statement is then parsed.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
#
use std::collections::HashSet;
use std::io::Write;
use std::process::{Command, Stdio};
#
# error_chain!{
#     errors { CmdError }
#     foreign_links {
#         Io(std::io::Error);
#         Utf8(std::string::FromUtf8Error);
#     }
# }

fn run() -> Result<()> {
    let mut child = Command::new("python").stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    child.stdin
        .as_mut()
        .ok_or("Child process stdin has not been captured!")?
        .write_all(b"import this; copyright(); credits(); exit()")?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout)?;
        let words = raw_output.split_whitespace()
            .map(|s| s.to_lowercase())
            .collect::<HashSet<_>>();
        println!("Found {} unique words:", words.len());
        println!("{:#?}", words);
        Ok(())
    } else {
        let err = String::from_utf8(output.stderr)?;
        bail!("External command failed:\n {}", err)
    }
}
#
# quick_main!(run);
```

[ex-run-piped-external-commands]: #ex-run-piped-external-commands
<a name="ex-run-piped-external-commands"></a>
## Run piped external commands

[![std-badge]][std] [![cat-os-badge]][cat-os]

Shows up to the 10<sup>th</sup> biggest files and subdirectories in
the current working directory. It is equivalent to run: `du -ah . |
sort -hr | head -n 10`.

It spawns Unix processes which are represented as [`Command`]s. In
order to capture the output of a child process it is necessary to
create a new [`Stdio::piped`] between parent and child.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
#
use std::process::{Command, Stdio};
#
# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#         Utf8(std::string::FromUtf8Error);
#     }
# }

fn run() -> Result<()> {
    let directory = std::env::current_dir()?;
    let du_output = Command::new("du")
        .arg("-ah")
        .arg(&directory)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| "Could not capture `du` standard output.")?;

    let sort_output = Command::new("sort")
        .arg("-hr")
        .stdin(du_output)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| "Could not capture `sort` standard output.")?;

    let head_output = Command::new("head")
        .args(&["-n", "10"])
        .stdin(sort_output)
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    println!(
        "Top 10 biggest files and directories in '{}':\n{}",
        directory.display(),
        String::from_utf8(head_output.stdout)?
    );

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

    buffered
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| set.is_match(line.as_str()))
        .for_each(|x| println!("{}", x));

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

        db.iter().enumerate().for_each(|(i, item)| println!("{}: {}", i, item));
        // release access
    }
    insert("grape")?;
    Ok(())
}
#
# quick_main!(run);
```

[ex-verify-extract-email]: #ex-verify-extract-email
<a name="ex-verify-extract-email"></a>
## Verify and extract login from an email address

[![regex-badge]][regex] [![lazy_static-badge]][lazy_static] [![cat-text-processing-badge]][cat-text-processing]

Validates that an email address is formatted correctly, and extracts everything
before the @ symbol.

```rust
#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

fn extract_login(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
            ^(?P<login>[^@\s]+)@
            ([[:word:]]+\.)*
            [[:word:]]+$
            ").unwrap();
    }
    RE.captures(input).and_then(|cap| {
        cap.name("login").map(|login| login.as_str())
    })
}

fn main() {
    assert_eq!(extract_login(r"I❤email@example.com"), Some(r"I❤email"));
    assert_eq!(
        extract_login(r"sdf+sdsfsd.as.sdsd@jhkk.d.rl"),
        Some(r"sdf+sdsfsd.as.sdsd")
    );
    assert_eq!(extract_login(r"More@Than@One@at.com"), None);
    assert_eq!(extract_login(r"Not an email@email"), None);
}
```

[ex-extract-hashtags]: #ex-extract-hashtags
<a name="ex-extract-hashtags"></a>
## Extract a list of unique #Hashtags from a text

[![regex-badge]][regex] [![lazy_static-badge]][lazy_static] [![cat-text-processing-badge]][cat-text-processing]

Extracts a sorted and deduplicated list of hashtags from a text.

The hashtag regex given here only catches Latin hashtags that start with a letter. The complete [twitter hashtag regex] is much more complicated.

```rust
extern crate regex;
#[macro_use]
extern crate lazy_static;

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

Replaces all occurrences of the hyphenated British English date pattern `2013-01-15`
with its equivalent slashed American English date pattern `01/15/2013`.

The method [`Regex::replace_all`] replaces all occurrences of the whole regex. The
`Replacer` trait helps to figure out the replacement string. This trait is implemented
for `&str` and allows to use variables like `$abcde` to refer to corresponding named capture groups
`(?P<abcde>REGEX)` from the search regex. See the [replacement string syntax] for examples
and information about escaping.

```rust
extern crate regex;
#[macro_use]
extern crate lazy_static;

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

[![regex-badge]][regex] [![cat-text-processing-badge]][cat-text-processing]

Processes a string of text using [`Regex::captures_iter`] to capture multiple
phone numbers.  The example here is for US convention phone numbers.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate regex;

use regex::Regex;
use std::fmt;
#
# error_chain!{
#     foreign_links {
#         Regex(regex::Error);
#         Io(std::io::Error);
#     }
# }

struct PhoneNumber<'a> {
    area: &'a str,
    exchange: &'a str,
    subscriber: &'a str,
}

// Allows printing phone numbers based on country convention.
impl<'a> fmt::Display for PhoneNumber<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "1 ({}) {}-{}", self.area, self.exchange, self.subscriber)
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
        r#"(?x)
          (?:\+?1)?                       # Country Code Optional
          [\s\.]?
          (([2-9]\d{2})|\(([2-9]\d{2})\)) # Area Code
          [\s\.\-]?
          ([2-9]\d{2})                    # Exchange Code
          [\s\.\-]?
          (\d{4})                         # Subscriber Number"#,
    )?;

    let phone_numbers = re.captures_iter(phone_text).filter_map(|cap| {
        // Area code populates either capture group 2 or 3.
        // Group 1 contains optional parenthesis.
        let groups = (cap.get(2).or(cap.get(3)), cap.get(4), cap.get(5));
        match groups {
            (Some(area), Some(ext), Some(sub)) => Some(PhoneNumber {
                area: area.as_str(),
                exchange: ext.as_str(),
                subscriber: sub.as_str(),
            }),
            _ => None,
        }
    });

    assert_eq!(
        phone_numbers.map(|m| m.to_string()).collect::<Vec<_>>(),
        vec![
            "1 (505) 881-9292",
            "1 (505) 778-2212",
            "1 (505) 881-9297",
            "1 (202) 991-9534",
            "1 (555) 392-0011",
            "1 (800) 233-2010",
            "1 (299) 339-1020",
        ]
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
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
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

[ex-hmac]: #ex-hmac
<a name="ex-hmac"></a>
## Sign and verify a message with HMAC digest

[![ring-badge]][ring] [![cat-cryptography-badge]][cat-cryptography]

Uses [`ring::hmac`] to creates a [`hmac::Signature`] of a string then verifies the signiture is correct.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate ring;
#
# error_chain! {
#     foreign_links {
#         Ring(ring::error::Unspecified);
#     }
# }

use ring::{digest, hmac, rand};
use ring::rand::SecureRandom;

fn run() -> Result<()> {
    let mut key_value = [0u8; 48];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut key_value)?;
    let key = hmac::SigningKey::new(&digest::SHA256, &key_value);

    let message = "Legitimate and important message.";
    let signature = hmac::sign(&key, message.as_bytes());
    hmac::verify_with_own_key(&key, message.as_bytes(), signature.as_ref())?;

    Ok(())
}
#
# quick_main!(run);
```

[ex-pbkdf2]: #ex-pbkdf2
<a name="ex-pbkdf2"></a>
## Salt and hash a password with PBKDF2

[![ring-badge]][ring] [![data-encoding-badge]][data-encoding] [![cat-cryptography-badge]][cat-cryptography]

Uses [`ring::pbkdf2`] to hash a salted password using the PBKDF2 key derivation
function [`pbkdf2::derive`] and then verifies the hash is correct with
[`pbkdf2::verify`]. The salt is generated using
[`SecureRandom::fill`], which fills the salt byte array with
securely generated random numbers.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate data_encoding;
extern crate ring;
#
# error_chain! {
#   foreign_links {
#     Ring(ring::error::Unspecified);
#   }
# }

use data_encoding::HEXUPPER;
use ring::{digest, pbkdf2, rand};
use ring::rand::SecureRandom;

fn run() -> Result<()> {
  const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
  const N_ITER: u32 = 100_000;
  let rng = rand::SystemRandom::new();

  // Generate salt
  let mut salt = [0u8; CREDENTIAL_LEN];
  rng.fill(&mut salt)?;

  // Hash password
  let password = "Guess Me If You Can!";
  let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
  pbkdf2::derive(
      &digest::SHA512,
      N_ITER,
      &salt,
      password.as_bytes(),
      &mut pbkdf2_hash,
  );
  println!("Salt: {}", HEXUPPER.encode(&salt));
  println!("PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));

  // Verify hash with correct password and wrong password
  let should_succeed = pbkdf2::verify(
      &digest::SHA512,
      N_ITER,
      &salt,
      password.as_bytes(),
      &pbkdf2_hash,
  );
  let wrong_password = "Definitely not the correct password";
  let should_fail = pbkdf2::verify(
      &digest::SHA512,
      N_ITER,
      &salt,
      wrong_password.as_bytes(),
      &pbkdf2_hash,
  );

  assert!(should_succeed.is_ok());
  assert!(!should_fail.is_ok());

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
        const FLAG_ABC     = Self::FLAG_A.bits
                           | Self::FLAG_B.bits
                           | Self::FLAG_C.bits;
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
    let e1 = MyFlags::FLAG_A | MyFlags::FLAG_C;
    let e2 = MyFlags::FLAG_B | MyFlags::FLAG_C;
    assert_eq!((e1 | e2), MyFlags::FLAG_ABC);   // union
    assert_eq!((e1 & e2), MyFlags::FLAG_C);     // intersection
    assert_eq!((e1 - e2), MyFlags::FLAG_A);     // set difference
    assert_eq!(!e2, MyFlags::FLAG_A);           // set complement

    let mut flags = MyFlags::FLAG_ABC;
    assert_eq!(format!("{}", flags), "00000000000000000000000000000111");
    assert_eq!(format!("{}", flags.clear()), "00000000000000000000000000000000");
    // Debug trait is automatically derived for the MyFlags through `bitflags!`
    assert_eq!(format!("{:?}", MyFlags::FLAG_B), "FLAG_B");
    assert_eq!(format!("{:?}", MyFlags::FLAG_A | MyFlags::FLAG_B), "FLAG_A | FLAG_B");
}
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

[ex-check-cpu-cores]: #ex-check-cpu-cores
<a name="ex-check-cpu-cores"></a>
## Check number of logical cpu cores

[![num_cpus-badge]][num_cpus] [![cat-hardware-support-badge]][cat-hardware-support]

Shows the number of logical cpu cores in current machine using [`num_cpus::get`].

```rust
extern crate num_cpus;

fn main() {
    println!("Number of logical cores is {}", num_cpus::get());
}
```

[ex-error-chain-backtrace]: #ex-error-chain-backtrace
<a name="ex-error-chain-backtrace"></a>
## Obtain backtrace of complex error scenarios

[![error-chain-badge]][error-chain] [![cat-rust-patterns-badge]][cat-rust-patterns]

This recipe shows how to handle a complex error scenario and then
print a backtrace. It relies on [`chain_err`] to extend errors by
appending new errors. The error stack can be unwinded, thus providing
a better context to understand why an error was raised.

The below recipes attemps to deserialize the value `256` into a
`u8`. An error will bubble up from Serde then csv and finaly up to the
user code.

```rust
# extern crate csv;
#[macro_use]
extern crate error_chain;
# #[macro_use]
# extern crate serde_derive;
#
# use std::fmt;
#
# error_chain! {
#     foreign_links {
#         Reader(csv::Error);
#     }
# }

#[derive(Debug, Deserialize)]
struct Rgb {
    red: u8,
    blue: u8,
    green: u8,
}

impl Rgb {
    fn from_reader(csv_data: &[u8]) -> Result<Rgb> {
        let color: Rgb = csv::Reader::from_reader(csv_data)
            .deserialize()
            .nth(0)
            .ok_or("Cannot deserialize the first CSV record")?
            .chain_err(|| "Cannot deserialize RGB color")?;

        Ok(color)
    }
}

# impl fmt::UpperHex for Rgb {
#     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
#         let hexa = u32::from(self.red) << 16 | u32::from(self.blue) << 8 | u32::from(self.green);
#         write!(f, "{:X}", hexa)
#     }
# }
#
fn run() -> Result<()> {
    let csv = "red,blue,green
102,256,204";

    let rgb = Rgb::from_reader(csv.as_bytes()).chain_err(|| "Cannot read CSV data")?;
    println!("{:?} to hexadecimal #{:X}", rgb, rgb);

    Ok(())
}

fn main() {
    if let Err(ref errors) = run() {
        eprintln!("Error level - description");
        errors
            .iter()
            .enumerate()
            .for_each(|(index, error)| eprintln!("└> {} - {}", index, error));

        if let Some(backtrace) = errors.backtrace() {
            eprintln!("{:?}", backtrace);
        }
#
#         // In a real use case, errors should handled. For example:
#         // ::std::process::exit(1);
    }
}
```

Backtrace error rendered:

```text
Error level - description
└> 0 - Cannot read CSV data
└> 1 - Cannot deserialize RGB color
└> 2 - CSV deserialize error: record 1 (line: 2, byte: 15): field 1: number too large to fit in target type
└> 3 - field 1: number too large to fit in target type
```

Run the recipe with `RUST_BACKTRACE=1` to display a detailed [`backtrace`] associated with this error.

[ex-measure-elapsed-time]: #ex-measure-elapsed-time
<a name="ex-measure-elapsed-time"></a>
## Measure the elapsed time between two code sections

[![std-badge]][std] [![cat-time-badge]][cat-time]

Measures [`time::Instant::elapsed`] since [`time::Instant::now`].

Calling [`time::Instant::elapsed`] returns a [`time::Duration`] that we print at the end of the example.
This method will not mutate or reset the [`time::Instant`] object.

```rust
use std::time::{Duration, Instant};
# use std::thread;
# 
# fn expensive_function() {
#     thread::sleep(Duration::from_secs(1));
# }

fn main() {
    let start = Instant::now();
    expensive_function();
    let duration = start.elapsed();
    
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
```

[ex-format-datetime]: #ex-format-datetime
<a name="ex-format-datetime"></a>
## Display formatted date and time
[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Gets and displays the current time in UTC using [`Utc::now`] in the well-known
formats RFC 2822 and RFC 3339, and in a custom format.

```rust
extern crate chrono;
use chrono::{DateTime, Utc};

fn main() {
    let now: DateTime<Utc> = Utc::now();

    println!("UTC now is: {}", now);
    println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    println!("UTC now in a custom format is: {}", now.format("%a %b %e %T %Y"));
}
```

{{#include links.md}}

<!-- API Reference -->

[`backtrace`]: https://docs.rs/error-chain/*/error_chain/trait.ChainedError.html#tymethod.backtrace
[`bitflags!`]: https://docs.rs/bitflags/*/bitflags/macro.bitflags.html
[`BufRead::lines`]: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
[`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html
[`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
[`chain_err`]: https://docs.rs/error-chain/*/error_chain/index.html#chaining-errors
[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`digest::Context`]: https://docs.rs/ring/*/ring/digest/struct.Context.html
[`digest::Digest`]: https://docs.rs/ring/*/ring/digest/struct.Digest.html
[`ring::hmac`]: https://docs.rs/ring/*/ring/hmac/
[`hmac::Signature`]: https://docs.rs/ring/*/ring/hmac/struct.Signature.html
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[`File::create`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.create
[`File::open`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.open
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[`IndependentSample::ind_sample`]: https://doc.rust-lang.org/rand/rand/distributions/trait.IndependentSample.html#tymethod.ind_sample
[`Lines`]: https://doc.rust-lang.org/std/io/struct.Lines.html
[`Mmap::as_slice`]: https://docs.rs/memmap/*/memmap/struct.Mmap.html#method.as_slice
[`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[`MutexGuard`]: https://doc.rust-lang.org/std/sync/struct.MutexGuard.html
[`Normal`]: https://doc.rust-lang.org/rand/rand/distributions/normal/struct.Normal.html
[`num_cpus::get`]: https://docs.rs/num_cpus/*/num_cpus/fn.get.html
[`time::Instant::now`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.now
[`time::Duration`]: https://doc.rust-lang.org/std/time/struct.Duration.html
[`time::Instant`]:https://doc.rust-lang.org/std/time/struct.Instant.html
[`time::Instant::elapsed`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.elapsed
[`Output`]: https://doc.rust-lang.org/std/process/struct.Output.html
[`pbkdf2::derive`]: https://docs.rs/ring/*/ring/pbkdf2/fn.derive.html
[`pbkdf2::verify`]: https://docs.rs/ring/*/ring/pbkdf2/fn.verify.html
[`rand::Rand`]: https://doc.rust-lang.org/rand/rand/trait.Rand.html
[`rand::Rand`]: https://doc.rust-lang.org/rand/rand/trait.Rand.html
[`rand::Rng`]: https://doc.rust-lang.org/rand/rand/trait.Rng.html
[`rand::thread_rng`]: https://doc.rust-lang.org/rand/rand/fn.thread_rng.html
[`Range`]: https://doc.rust-lang.org/rand/rand/distributions/range/struct.Range.html
[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`Regex::captures_iter`]: https://doc.rust-lang.org/regex/regex/struct.Regex.html#method.captures_iter
[`regex::RegexSet`]: https://doc.rust-lang.org/regex/regex/struct.RegexSet.html
[`regex::RegexSetBuilder`]: https://doc.rust-lang.org/regex/regex/struct.RegexSetBuilder.html
[`Regex::replace_all`]: https://docs.rs/regex/*/regex/struct.Regex.html#method.replace_all
[`Regex`]: https://doc.rust-lang.org/regex/regex/struct.Regex.html
[`ring::pbkdf2`]: https://docs.rs/ring/*/ring/pbkdf2/index.html
[`Rng::gen_range`]: https://doc.rust-lang.org/rand/rand/trait.Rng.html#method.gen_range
[`RwLock`]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
[`SecureRandom::fill`]: https://docs.rs/ring/*/ring/rand/trait.SecureRandom.html#tymethod.fill
[`seek`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.seek
[`Stdio::piped`]: https://doc.rust-lang.org/std/process/struct.Stdio.html
[`Utc::now`]: https://docs.rs/chrono/*/chrono/offset/struct.Utc.html#method.now
[rand-distributions]: https://doc.rust-lang.org/rand/rand/distributions/index.html
[replacement string syntax]: https://docs.rs/regex/*/regex/struct.Regex.html#replacement-string-syntax

<!-- Other Reference -->

[race-condition-file]: https://en.wikipedia.org/wiki/Race_condition#File_systems
[raw string literals]: https://doc.rust-lang.org/reference/tokens.html#raw-string-literals
[twitter hashtag regex]: https://github.com/twitter/twitter-text/blob/c9fc09782efe59af4ee82855768cfaf36273e170/java/src/com/twitter/Regex.java#L255
[uniform distribution]: https://en.wikipedia.org/wiki/Uniform_distribution_(continuous)
