# Application development

| Recipe | Crates |
|--------|--------|
| [Parse command line arguments][ex-clap-basic] | [![clap-badge]][clap] |

[ex-clap-basic]: #ex-clap-basic
<a name="ex-clap-basic"></a>
## Parse command line arguments

[![clap-badge]][clap]

```rust
// Argument parsing
extern crate clap;
use clap::{Arg, App};

fn main() {

    // Set command line arguments
    let matches = App::new("My Test Program")
                           .version("0.0.1")
                           .author("Hackerman Jones <hckrmnjones@hack.gov>")
                           .about("Teaches argument parsing")
                           .arg(Arg::with_name("file")
                                .short("f")
                                .long("file")
                                .value_name("FILE")
                                .help("A cool file")
                                .takes_value(true))
                           .arg(Arg::with_name("input_string")
                                .short("i")
                                .help("Your favorite phrase")
                                .multiple(true)
                                .takes_value(true)
                                .index(1))
                           .arg(Arg::with_name("count")
                                .short("c")
                                .long("count")
                                .takes_value(true)
                                .help("5 less than your favorite number"))
                           .get_matches();

    // Gets value for file, or defaults to 'input.txt'
    let myfile = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", myfile);

    // Gets value for input string, simple unwrap is safe,
    // as this arg was required
    let mystring = matches.value_of("input_string").unwrap_or("");
    println!("The string passed is: {}", mystring);

    // Gets value for count, default to 0, then convert to int
    let count_string = matches.value_of("count").unwrap_or("0");
    let mut count = count_string.parse::<i32>().unwrap_or(0);

    count += 5;

    println!("The value of the number passed plus 5 is: {}", count);
}
```

The purpose of this is to display the raw power of the clap library, an
extensive command line argument parser for Rust. First, the user writes a
description for their App, given a version, author, and about string.

The next step involves describing the args that clap can expect. 'with_name' is
the option or argument descriptor that 'value_of' uses to get the value passed.
'short' and 'long' sets the version to be passed with characters '-' and '--',
respectively, through the command line. 'value_name' is simply cosmetic, and is
NOT used in getting any actual values. 'help' allows the user to describe what
the argument is used for or what the argument expects. If 'takes_value' is set
to true, it will expect the argument to take a value, such as a string,
filename, or number, as in the example above. For more examples of usage and a
description of what clap is capable of, please visit their
[wiki](https://kbknapp.github.io/clap-rs/clap/struct.App.html).

We can test this program by running command similar to the following:

```
cargo run -- -f myfile.txt -i "I <3 Rust!!!" --count 6
```

```
> The file passed is: myfile.txt
> The string passed is: I <3 Rust!!!
> The value of the number passed plus 5 is: 11
```

<!-- Crates -->

[clap-badge]: https://img.shields.io/crates/v/clap.svg?label=clap
[clap]: https://docs.rs/clap/
