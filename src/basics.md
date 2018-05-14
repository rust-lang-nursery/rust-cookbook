# Basics

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Read lines of strings from a file][ex-std-read-lines] | [![std-badge]][std] | [![cat-filesystem-badge]][cat-filesystem] |
| [Read and write integers in little-endian byte order][ex-byteorder-le] | [![byteorder-badge]][byteorder] | [![cat-encoding-badge]][cat-encoding] |
| [Generate random numbers][ex-rand] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random numbers within a range][ex-rand-range] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random numbers with given distribution][ex-rand-dist] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Generate random values of a custom type][ex-rand-custom] | [![rand-badge]][rand] | [![cat-science-badge]][cat-science] |
| [Create random passwords from a set of alphanumeric characters][ex-rand-passwd] | [![rand-badge]][rand] | [![cat-os-badge]][cat-os] |
| [Create random passwords from a set of user-defined characters][ex-rand-choose] | [![rand-badge]][rand] | [![cat-os-badge]][cat-os] |
| [Run an external command and process stdout][ex-parse-subprocess-output] | [![regex-badge]][regex] | [![cat-os-badge]][cat-os] [![cat-text-processing-badge]][cat-text-processing] |
| [Run an external command passing it stdin and check for an error code][ex-parse-subprocess-input] | [![regex-badge]][regex] | [![cat-os-badge]][cat-os] [![cat-text-processing-badge]][cat-text-processing] |
| [Run piped external commands][ex-run-piped-external-commands] | [![std-badge]][std] | [![cat-os-badge]][cat-os] |
| [Redirect both stdout and stderr of child process to the same file][ex-redirect-stdout-stderr-same-file] | [![std-badge]][std] | [![cat-os-badge]][cat-os] |
| [Continuously process child process' outputs][ex-continuous-process-output] | [![std-badge]][std] | [![cat-os-badge]][cat-os][![cat-text-processing-badge]][cat-text-processing] |
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
| [Handle errors correctly in main][ex-error-chain-simple-error-handling] | [![error-chain-badge]][error-chain] | [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Avoid discarding errors during error conversions][ex-error-chain-avoid-discarding] | [![error-chain-badge]][error-chain] | [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Obtain backtrace of complex error scenarios][ex-error-chain-backtrace] | [![error-chain-badge]][error-chain] | [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Measure elapsed time][ex-measure-elapsed-time] | [![std-badge]][std] | [![cat-time-badge]][cat-time] |
| [Convert date to UNIX timestamp and vice versa][ex-convert-datetime-timestamp] | [![chrono-badge]][chrono] | [![cat-date-and-time-badge]][cat-date-and-time] |
| [Convert a local time to an another UTC timezone and vice versa][ex-convert-datetime-timezone] | [![chrono-badge]][chrono] | [![cat-date-and-time-badge]][cat-date-and-time] |
| [Display formatted date and time][ex-format-datetime] | [![chrono-badge]][chrono] | [![cat-date-and-time-badge]][cat-date-and-time] |
| [Parse string into DateTime struct][ex-parse-datetime] | [![chrono-badge]][chrono] | [![cat-date-and-time-badge]][cat-date-and-time] |
| [Perform checked date and time calculations][ex-datetime-arithmetic] | [![chrono-badge]][chrono] | [![cat-date-and-time-badge]][cat-date-and-time] |
| [Examine the date and time][ex-examine-date-and-time] | [![chrono-badge]][chrono] | [![cat-date-and-time-badge]][cat-date-and-time] |
| [File names that have been modified in the last 24 hours for the working directory][ex-file-24-hours-modified] | [![std-badge]][std] | [![cat-filesystem-badge]][cat-filesystem] [![cat-os-badge]][cat-os] |

[ex-std-read-lines]: #ex-std-read-lines
<a name="ex-std-read-lines"></a>
{{#include file/read-write/read-file.md}}

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

{{#include algorithms/randomness/rand.md}}

{{#include algorithms/randomness/rand-range.md}}

{{#include algorithms/randomness/rand-dist.md}}

<<<<<<< HEAD
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

use rand::distributions::{Range, Distribution};

fn main() {
    let mut rng = rand::thread_rng();
    let die = Range::new(1, 7);

    loop {
        let throw = die.sample(&mut rng);
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
[`Distribution::sample`] with help of a random-number
generator [`rand::Rng`].

The [distributions available are documented here][rand-distributions]. An example using the
[`Normal`] distribution is shown below.

```rust
extern crate rand;

use rand::distributions::{Normal, Distribution};

fn main() {
    let mut rng = rand::thread_rng();

    // mean 2, standard deviation 3:
    let normal = Normal::new(2.0, 3.0);
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v)
}
```

[ex-rand-custom]: #ex-rand-custom
<a name="ex-rand-custom"></a>
## Generate random values of a custom type

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

Randomly generates a tuple `(i32, bool, f64)` and variable of user defined type `Point`.
Implements the [`Distribution`] trait on type `Point` for [`Standard`] in order to allow random generation.

```rust
extern crate rand;

use rand::Rng;
use rand::distributions::{Distribution, Standard};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
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

[ex-rand-passwd]: #ex-rand-passwd
<a name="ex-rand-passwd"></a>
## Create random passwords from a set of alphanumeric characters

[![rand-badge]][rand] [![cat-os-badge]][cat-os]

Randomly generates a string of given length ASCII characters in the range `A-Z, a-z, 0-9`, with [`Alphanumeric`] sample.

```rust
extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

    println!("{}", rand_string);
}
```

[ex-rand-choose]: #ex-rand-choose
<a name="ex-rand-choose"></a>
## Create random passwords from a set of user-defined characters

[![rand-badge]][rand] [![cat-os-badge]][cat-os]

Randomly generates a string of given length ASCII characters with custom user-defined bytestring, with [`choose`].

```rust
extern crate rand;

use rand::{thread_rng, Rng};

fn main() {
    const CHARSET: &[u8] =  b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";

    let mut rng = thread_rng();
    let password: Option<String> = (0..30)
        .map(|_| Some(*rng.choose(CHARSET)? as char))
        .collect();

    println!("{:?}", password);
}
```
=======
{{#include algorithms/randomness/rand-custom.md}}

{{#include algorithms/randomness/rand-passwd.md}}

{{#include algorithms/randomness/rand-choose.md}}
>>>>>>> dbca385... Add algorithms

[ex-parse-subprocess-output]: #ex-parse-subprocess-output
<a name="ex-parse-subprocess-output"></a>
{{#include os/external/process-output.md}}

[ex-parse-subprocess-input]: #ex-parse-subprocess-input
<a name="ex-parse-subprocess-input"></a>
{{#include os/external/send-input.md}}

[ex-run-piped-external-commands]: #ex-run-piped-external-commands
<a name="ex-run-piped-external-commands"></a>
{{#include os/external/piped.md}}

[ex-redirect-stdout-stderr-same-file]: #ex-redirect-stdout-stderr-same-file
<a name="ex-redirect-stdout-stderr-same-file"></a>
{{#include os/external/error-file.md}}

[ex-continuous-process-output]: #ex-continuous-process-output
<a name="ex-continuous-process-output"></a>
{{#include os/external/continuous.md}}

[ex-regex-filter-log]: #ex-regex-filter-log
<a name="ex-regex-filter-log"></a>
{{#include text/regex/filter-log.md}}

[ex-lazy-constant]: #ex-lazy-constant
{{#include data_structures/constant/lazy-constant.md}}

{{#include concurrency/thread/global-mut-state.md}}

[ex-verify-extract-email]: #ex-verify-extract-email
<a name="ex-verify-extract-email"></a>
{{#include text/regex/email.md}}

[ex-extract-hashtags]: #ex-extract-hashtags
<a name="ex-extract-hashtags"></a>
{{#include text/regex/hashtags.md}}

[ex-regex-replace-named]: #ex-regex-replace-named
<a name="ex-regex-replace-named"></a>
{{#include text/regex/replace.md}}

[ex-phone]: #ex-phone
<a name="ex-phone"></a>
{{#include text/regex/phone.md}}

{{#include cryptography/hashing/sha-digest.md}}

{{#include cryptography/hashing/hmac.md}}

{{#include cryptography/encryption/pbkdf2.md}}

{{#include data_structures/custom/bitfield.md}}

[ex-random-file-access]: #ex-random-file-access
<a name="ex-random-file-access"></a>

{{#include file/read-write/memmap.md}}

[ex-check-cpu-cores]: #ex-check-cpu-cores
<a name="ex-check-cpu-cores"></a>
{{#include hardware/processor/cpu-count.md}}

[ex-error-chain-simple-error-handling]: #ex-error-chain-simple-error-handling
<a name="ex-error-chain-simple-error-handling"></a>
## Handle errors correctly in main

[![error-chain-badge]][error-chain] [![cat-rust-patterns-badge]][cat-rust-patterns]

Handles error that occur when trying to open a file that does not
exist. It is achieved by using [error-chain], a library that takes
care of a lot of boilerplate code needed in order to [handle errors in
Rust].

`Io(std::io::Error)` inside [`foreign_links`] allows automatic
conversion from [`std::io::Error`] into [`error_chain!`] defined type
implementing the [`Error`] trait.

The below recipe will tell how long the system has been running by
opening the Unix file `/proc/uptime` and parse the content to get the
first number. Returns uptime unless there is an error.

```rust
#[macro_use]
extern crate error_chain;

use std::fs::File;
use std::io::Read;

error_chain!{
    foreign_links {
        Io(std::io::Error);
        ParseInt(::std::num::ParseIntError);
    }
}

fn read_uptime() -> Result<u64> {
    let mut uptime = String::new();
    File::open("/proc/uptime")?.read_to_string(&mut uptime)?;

    Ok(uptime
        .split('.')
        .next()
        .ok_or("Cannot parse uptime data")?
        .parse()?)
}

fn main() {
    match read_uptime() {
        Ok(uptime) => println!("uptime: {} seconds", uptime),
        Err(err) => eprintln!("error: {}", err),
    };
}
```

[ex-error-chain-avoid-discarding]: #ex-error-chain-avoid-discarding
<a name="ex-error-chain-avoid-discarding"></a>
## Avoid discarding errors during error conversions

[![error-chain-badge]][error-chain] [![cat-rust-patterns-badge]][cat-rust-patterns]

[Matching] on different error types returned by a function is possible
and relatively compact with [error-chain] crate. To do so,
[`ErrorKind`] is used to determine the error type.

To illustrate the error matching, a random integer generator web
service will be queried via [reqwest] and then the response will be
parsed. Errors can be generated by the Rust standard library,
[reqwest] and by the web service. [`foreign_links`] are used for well
defined Rust errors. The `errors` block of the `error_chain!` macro is
used to create an additional [`ErrorKind`] variant for the web service
error. Finally, a regular `match` can be used to react differently
according to the raised error.

```rust
#[macro_use]
extern crate error_chain;
extern crate reqwest;

use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
        ParseIntError(std::num::ParseIntError);
    }

    errors { RandomResponseError(t: String) }
}

fn parse_response(mut response: reqwest::Response) -> Result<u32> {
    let mut body = String::new();
    response.read_to_string(&mut body)?;
    body.pop();
    body.parse::<u32>()
        .chain_err(|| ErrorKind::RandomResponseError(body))
}

fn run() -> Result<()> {
    let url =
        format!("https://www.random.org/integers/?num=1&min=0&max=10&col=1&base=10&format=plain");
    let response = reqwest::get(&url)?;
    let random_value: u32 = parse_response(response)?;

    println!("a random number between 0 and 10: {}", random_value);

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        match *error.kind() {
            ErrorKind::Io(_) => println!("Standard IO error: {:?}", error),
            ErrorKind::Reqwest(_) => println!("Reqwest error: {:?}", error),
            ErrorKind::ParseIntError(_) => println!("Standard parse int error: {:?}", error),
            ErrorKind::RandomResponseError(_) => println!("User defined error: {:?}", error),
            _ => println!("Other error: {:?}", error),
        }
    }
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

The below recipes attempts to deserialize the value `256` into a
`u8`. An error will bubble up from Serde then csv and finally up to the
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
[ex-convert-datetime-timezone]: #ex-convert-datetime-timezone
<a name="ex-convert-datetime-timezone"></a>
## Convert a local time to an another UTC timezone and vice versa
[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Gets the local time and displays it using [`offset::Local::now`] and then converts it to the UTC standard using the [`DateTime::from_utc`] struct method. A time is then converted using the [`offset::FixedOffset`] struct and the UTC time is then converted to UTC+8 and UTC-2.

```rust
extern crate chrono;

use chrono::{DateTime, FixedOffset, Local, Utc};

fn main() {
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    let china_timezone = FixedOffset::east(8 * 3600);
    let rio_timezone = FixedOffset::west(2 * 3600);
    println!("Local time now is {}", local_time);
    println!("UTC time now is {}", utc_time);
    println!(
        "Time in Hong Kong now is {}",
        utc_time.with_timezone(&china_timezone)
    );
    println!("Time in Rio de Janeiro now is {}", utc_time.with_timezone(&rio_timezone));
}
```

[ex-convert-datetime-timestamp]: #ex-convert-datetime-timestamp
<a name="ex-convert-datetime-timestamp"></a>
## Convert date to UNIX timestamp and vice versa
[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Converts a date given by [`NaiveDate::from_ymd`] and [`NaiveTime::from_hms`]
to [UNIX timestamp] using [`NaiveDateTime::timestamp`].
Then it calculates what was the date after one billion seconds
since January 1, 1970 0:00:00 UTC, using [`NaiveDateTime::from_timestamp`].

```rust
extern crate chrono;

use chrono::{NaiveDate, NaiveDateTime};

fn main() {
    let date_time: NaiveDateTime = NaiveDate::from_ymd(2017, 11, 12).and_hms(17, 33, 44);
    println!(
        "Number of seconds between 1970-01-01 00:00:00 and {} is {}.",
        date_time, date_time.timestamp());

    let date_time_after_a_billion_seconds = NaiveDateTime::from_timestamp(1_000_000_000, 0);
    println!(
        "Date after a billion seconds since 1970-01-01 00:00:00 was {}.",
        date_time_after_a_billion_seconds);
}
```

[ex-format-datetime]: #ex-format-datetime
<a name="ex-format-datetime"></a>
## Display formatted date and time
[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Gets and displays the current time in UTC using [`Utc::now`]. Formats the
current time in the well-known formats [RFC 2822] using [`DateTime::to_rfc2822`]
and [RFC 3339] using [`DateTime::to_rfc3339`], and in a custom format using
[`DateTime::format`].

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

[ex-parse-datetime]: #ex-parse-datetime
<a name="ex-parse-datetime"></a>
## Parse string into DateTime struct
[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Parses a [`DateTime`] struct from strings representing the well-known formats
[RFC 2822], [RFC 3339], and a custom format, using
[`DateTime::parse_from_rfc2822`], [`DateTime::parse_from_rfc3339`], and
[`DateTime::parse_from_str`] respectively.

Escape sequences that are available for the [`DateTime::parse_from_str`] can be
found at [`chrono::format::strftime`]. Note that the [`DateTime::parse_from_str`]
requires that such a DateTime struct must be creatable that it uniquely
identifies a date and a time. For parsing dates and times without timezones use
[`NaiveDate`], [`NaiveTime`], and [`NaiveDateTime`].

```rust
extern crate chrono;
# #[macro_use]
# extern crate error_chain;
#
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
#
# error_chain! {
#     foreign_links {
#         DateParse(chrono::format::ParseError);
#     }
# }

fn run() -> Result<()> {
    let rfc2822 = DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200")?;
    println!("{}", rfc2822);

    let rfc3339 = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")?;
    println!("{}", rfc3339);

    let custom = DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z")?;
    println!("{}", custom);

    let time_only = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S")?;
    println!("{}", time_only);

    let date_only = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
    println!("{}", date_only);

    let no_timezone = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")?;
    println!("{}", no_timezone);

    Ok(())
}
#
# quick_main!(run);
```

[ex-datetime-arithmetic]: #ex-datetime-arithmetic
<a name="ex-datetime-arithmetic"></a>
## Perform checked date and time calculations
[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Calculates and displays the date and time two weeks from now using
[`DateTime::checked_add_signed`] and the date of the day before that using
[`DateTime::checked_sub_signed`]. The methods return None if the date and time
cannot be calculated.

Escape sequences that are available for the
[`DateTime::format`] can be found at [`chrono::format::strftime`].

```rust
extern crate chrono;
use chrono::{DateTime, Duration, Utc};

fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_sub_signed(Duration::days(1))
}

fn main() {
    let now = Utc::now();
    println!("{}", now);

    let almost_three_weeks_from_now = now.checked_add_signed(Duration::weeks(2))
            .and_then(|in_2weeks| in_2weeks.checked_add_signed(Duration::weeks(1)))
            .and_then(day_earlier);

    match almost_three_weeks_from_now {
        Some(x) => println!("{}", x),
        None => eprintln!("Almost three weeks from now overflows!"),
    }

    match now.checked_add_signed(Duration::max_value()) {
        Some(x) => println!("{}", x),
        None => eprintln!("We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."),
    }
}
```

[ex-examine-date-and-time]: #ex-examine-date-and-time
<a name="ex-examine-date-and-time"></a>
## Examine the date and time
[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Gets the current UTC [`DateTime`] and its hour/minute/second via [`Timelike`]
and its year/month/day/weekday via [`Datelike`].

```rust
extern crate chrono;
use chrono::{Datelike, Timelike, Utc};

fn main() {
    let now = Utc::now();

    let (is_pm, hour) = now.hour12();
    println!(
        "The current UTC time is {:02}:{:02}:{:02} {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );
    println!(
        "And there have been {} seconds since midnight",
        now.num_seconds_from_midnight()
    );

    let (is_common_era, year) = now.year_ce();
    println!(
        "The current UTC date is {}-{:02}-{:02} {:?} ({})",
        year,
        now.month(),
        now.day(),
        now.weekday(),
        if is_common_era { "CE" } else { "BCE" }
    );
    println!(
        "And the Common Era began {} days ago",
        now.num_days_from_ce()
    );
}
```

[ex-file-24-hours-modified]: #ex-file-24-hours-modified
<a name="ex-file-24-hours-modified"></a>
## File names that have been modified in the last 24 hours for the working directory

[![std-badge]][std] [![cat-filesystem-badge]][cat-filesystem]

Gets the current working directory by calling [`env::current_dir`],
then for each entries in [`fs::read_dir`], extracts the
[`DirEntry::path`] and gets the metada via [`fs::Metadata`]. The
[`Metadata::modified`] returns the [`SystemTime::elapsed`] time since
last modification of the entry. It's converted into seconds with
[`Duration::as_secs`] and compared with 24 hours (24 * 60 * 60
seconds). [`Metadata::is_file`] is used to filter out directories.

```rust
# #[macro_use]
# extern crate error_chain;
#
use std::{env, fs};

# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#         SystemTimeError(std::time::SystemTimeError);
#     }
# }
#
fn run() -> Result<()> {
    let current_dir = env::current_dir()?;
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs();

        if last_modified < 24 * 3600 && metadata.is_file() {
            println!(
                "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename")?
            );
        }
    }

    Ok(())
}
#
# quick_main!(run);
```

{{#include links.md}}

<!-- API Reference -->

[`backtrace`]: https://docs.rs/error-chain/*/error_chain/trait.ChainedError.html#tymethod.backtrace
[`bitflags!`]: https://docs.rs/bitflags/*/bitflags/macro.bitflags.html
[`BufRead::lines`]: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
[`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html
[`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
[`chain_err`]: https://docs.rs/error-chain/*/error_chain/index.html#chaining-errors
[`choose`]: https://docs.rs/rand/*/rand/trait.Rng.html#method.choose
[`chrono::format::strftime`]: https://docs.rs/chrono/*/chrono/format/strftime/index.html
[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`Datelike`]: https://docs.rs/chrono/*/chrono/trait.Datelike.html
[`DateTime::checked_add_signed`]: https://docs.rs/chrono/*/chrono/struct.Date.html#method.checked_add_signed
[`DateTime::checked_sub_signed`]: https://docs.rs/chrono/*/chrono/struct.Date.html#method.checked_sub_signed
[`DateTime::format`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.format
[`DateTime::format`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.format
[`DateTime::format`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.format
[`DateTime::from_utc`]:https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.from_utc
[`DateTime::parse_from_rfc2822`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.parse_from_rfc2822
[`DateTime::parse_from_rfc3339`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.parse_from_rfc3339
[`DateTime::parse_from_str`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.parse_from_str
[`DateTime::to_rfc2822`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.to_rfc2822
[`DateTime::to_rfc3339`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.to_rfc3339
[`DateTime`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html
[`digest::Context`]: https://briansmith.org/rustdoc/ring/digest/struct.Context.html
[`digest::Digest`]: https://briansmith.org/rustdoc/ring/digest/struct.Digest.html
[`DirEntry::path`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.path
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[`Duration::as_secs`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.as_secs
[`env::current_dir`]: https://doc.rust-lang.org/std/env/fn.current_dir.html
[`error_chain!`]: https://docs.rs/error-chain/*/error_chain/macro.error_chain.html
[`Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
[`ErrorKind`]: https://docs.rs/error-chain/*/error_chain/example_generated/enum.ErrorKind.html
[`File::create`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.create
[`File::open`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.open
[`File::try_clone`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.try_clone
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`foreign_links`]: https://docs.rs/error-chain/*/error_chain/#foreign-links
[`fs::Metadata`]: https://doc.rust-lang.org/std/fs/struct.Metadata.html
[`fs::read_dir`]: https://doc.rust-lang.org/std/fs/fn.read_dir.html
[`Alphanumeric`]: https://docs.rs/rand/*/rand/distributions/struct.Alphanumeric.html#struct.Alphanumaric
[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[`hmac::Signature`]: https://briansmith.org/rustdoc/ring/hmac/struct.Signature.html
[`Distribution::sample`]: https://docs.rs/rand/*/rand/distributions/trait.Distribution.html#tymethod.sample
[`Lines`]: https://doc.rust-lang.org/std/io/struct.Lines.html
[`Metadata::is_file`]: https://doc.rust-lang.org/std/fs/struct.Metadata.html#method.is_file
[`Metadata::modified`]: https://doc.rust-lang.org/std/fs/struct.Metadata.html#method.modified
[`Mmap::map`]: https://docs.rs/memmap/*/memmap/struct.Mmap.html#method.map
[`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[`MutexGuard`]: https://doc.rust-lang.org/std/sync/struct.MutexGuard.html
[`NaiveDate::from_ymd`]: https://docs.rs/chrono/*/chrono/naive/struct.NaiveDate.html#method.from_ymd
[`NaiveDate`]: https://docs.rs/chrono/*/chrono/naive/struct.NaiveDate.html
[`NaiveDateTime::from_timestamp`]: https://docs.rs/chrono/*/chrono/naive/struct.NaiveDateTime.html#method.from_timestamp
[`NaiveDateTime::timestamp`]: https://docs.rs/chrono/*/chrono/naive/struct.NaiveDateTime.html#method.timestamp
[`NaiveDateTime`]: https://docs.rs/chrono/*/chrono/naive/struct.NaiveDateTime.html
[`NaiveTime::from_hms`]: https://docs.rs/chrono/*/chrono/naive/struct.NaiveTime.html#method.from_hms
[`NaiveTime`]: https://docs.rs/chrono/*/chrono/naive/struct.NaiveTime.html
[`Normal`]: https://docs.rs/rand/*/rand/distributions/normal/struct.Normal.html
[`num_cpus::get`]: https://docs.rs/num_cpus/*/num_cpus/fn.get.html
[`offset::FixedOffset`]: https://docs.rs/chrono/*/chrono/offset/struct.FixedOffset.html
[`offset::Local::now`]: https://docs.rs/chrono/*/chrono/offset/struct.Local.html#method.now
[`Output`]: https://doc.rust-lang.org/std/process/struct.Output.html
[`pbkdf2::derive`]: https://briansmith.org/rustdoc/ring/pbkdf2/fn.derive.html
[`pbkdf2::verify`]: https://briansmith.org/rustdoc/ring/pbkdf2/fn.verify.html
[`process::Stdio`]: https://doc.rust-lang.org/std/process/struct.Stdio.html
[`rand::Rng`]: https://docs.rs/rand/*/rand/trait.Rng.html
[`rand::thread_rng`]: https://docs.rs/rand/*/rand/fn.thread_rng.html
[`Range`]: https://docs.rs/rand/*/rand/distributions/#reexports
[`Standard`]: https://docs.rs/rand/*/rand/distributions/struct.Standard.html
[`Distribution`]: https://docs.rs/rand/*/rand/distributions/trait.Distribution.html
[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`Regex::captures_iter`]: https://doc.rust-lang.org/regex/*/regex/struct.Regex.html#method.captures_iter
[`regex::RegexSet`]: https://doc.rust-lang.org/regex/*/regex/struct.RegexSet.html
[`regex::RegexSetBuilder`]: https://doc.rust-lang.org/regex/*/regex/struct.RegexSetBuilder.html
[`Regex::replace_all`]: https://docs.rs/regex/*/regex/struct.Regex.html#method.replace_all
[`Regex`]: https://doc.rust-lang.org/regex/*/regex/struct.Regex.html
[`ring::hmac`]: https://briansmith.org/rustdoc/ring/hmac/
[`ring::pbkdf2`]: https://briansmith.org/rustdoc/ring/pbkdf2/index.html
[`Rng::gen_range`]: https://docs.rs/rand/*/rand/trait.Rng.html#method.gen_range
[`RwLock`]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
[`SecureRandom::fill`]: https://briansmith.org/rustdoc/ring/rand/trait.SecureRandom.html#tymethod.fill
[`seek`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.seek
[`std::io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
[`Stdio::piped`]: https://doc.rust-lang.org/std/process/struct.Stdio.html
[`SystemTime::elapsed`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.elapsed
[`time::Duration`]: https://doc.rust-lang.org/std/time/struct.Duration.html
[`time::Instant::elapsed`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.elapsed
[`time::Instant::now`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.now
[`time::Instant`]:https://doc.rust-lang.org/std/time/struct.Instant.html
[`Timelike`]: https://docs.rs/chrono/*/chrono/trait.Timelike.html
[`Utc::now`]: https://docs.rs/chrono/*/chrono/offset/struct.Utc.html#method.now
[Matching]:https://docs.rs/error-chain/*/error_chain/#matching-errors
[rand-distributions]: https://docs.rs/rand/*/rand/distributions/index.html
[replacement string syntax]: https://docs.rs/regex/*/regex/struct.Regex.html#replacement-string-syntax

<!-- Other Reference -->

[handle errors in Rust]: https://doc.rust-lang.org/book/second-edition/ch09-00-error-handling.html
[race-condition-file]: https://en.wikipedia.org/wiki/Race_condition#File_systems
[raw string literals]: https://doc.rust-lang.org/reference/tokens.html#raw-string-literals
[RFC 2822]: https://www.ietf.org/rfc/rfc2822.txt
[RFC 3339]: https://www.ietf.org/rfc/rfc3339.txt
[twitter hashtag regex]: https://github.com/twitter/twitter-text/blob/c9fc09782efe59af4ee82855768cfaf36273e170/java/src/com/twitter/Regex.java#L255
[uniform distribution]: https://en.wikipedia.org/wiki/Uniform_distribution_(continuous)
[UNIX timestamp]: https://en.wikipedia.org/wiki/Unix_time
