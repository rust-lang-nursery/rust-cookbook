# Clap Argument Validation

Not every argument can be trusted simply because it parses into the right type. A string might
convert fine into a [`PathBuf`] or a number, while still being meaningless in context, such as a
percentage over 100 or a path that does not point to an existing file. Clap addresses this through
the [`value_parser`] attribute, which lets you attach custom validation logic to an argument beyond
basic type conversion. This recipe uses a disk-cleanup tool to demonstrate three common validation
patterns: numeric ranges, file existence checks, and custom string formats.

For numeric ranges, Clap provides a built-in solution. The `threshold` field uses
`value_parser!(u8).range(0..=100)`, which rejects any value outside that range before the program
even runs, removing the need for manual bounds checking in your own code.

For validation that goes beyond simple ranges, you can write your own parser function and pass it to
`value_parser`. The `config` field uses `parse_config`, which checks that the given path actually
exists as a file. Similarly, the `notify` field uses `parse_email`, which performs a basic
structural check on the input. Refer to this [`recipe`] for concrete email validation. Both
functions return a `Result<T, String>`, where the error message becomes part of the output Clap
shows the user when validation fails, making it clear which argument was invalid and why.

> This recipe requires the [`derive`] feature flag to be enabled in `Cargo.toml`.

```rust,edition2024,no_run
use clap::{Parser, value_parser};
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct CliArgs {
    /// Path to scan for cleanup
    #[arg(short = 'p', long = "path")]
    path: PathBuf,

    /// Free space until usage falls below this percentage
    #[arg(short = 't', long = "threshold", value_parser = value_parser!(u8).range(0..=100))]
    threshold: u8,

    /// Path to config file
    #[arg(short = 'c', long = "config", value_parser = parse_config)]
    config: PathBuf,

    /// Email to notify on completion
    #[arg(short = 'n', long = "notify", value_parser = parse_email)]
    notify: String,
}

fn parse_email(s: &str) -> Result<String, String> {
    match s.split_once('@') {
        Some((user, domain)) if !user.is_empty() && domain.contains('.') => Ok(s.to_string()),
        _ => Err("Invalid email format".to_string()),
    }
}

fn parse_config(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if path.is_file() {
        Ok(path)
    } else {
        Err("Provided config path is not a file".to_string())
    }
}

fn main() {
    let args = CliArgs::parse();
    dbg!(args);
}
```

[`PathBuf`]: https://doc.rust-lang.org/std/path/struct.PathBuf.html
[`derive`]: https://docs.rs/clap/*/clap/_features/index.html
[`recipe`]: https://rust-lang-nursery.github.io/rust-cookbook/text/regex.html#verify-and-extract-login-from-an-email-address
[`value_parser`]: https://docs.rs/clap/*/clap/macro.value_parser.html
