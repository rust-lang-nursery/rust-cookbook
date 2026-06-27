# Clap Derive Parser Macro

Clap's [`Parser`] derive macro is the most common way to define a command line interface in Rust.
Instead of manually parsing `std::env::args()`, you describe the shape of your arguments as a
struct, and Clap generates all the parsing logic for you. This example defines the arguments for a
backend server, including a required server URL and several optional settings.

Whether an argument is required or optional depends on its type. Wrapping a field in `Option<T>`
makes it optional, while using `T` directly makes it required. In this example, `server_url` is the
only required argument, while `log_level`, `server_port`, and `snapshot_frequency` may all be
omitted when running the program.

Since command line input always arrives as plain text, custom types like `LogLevel` need a way to be
constructed from a string. This is done here through a `From<String>` implementation, which converts
the raw argument into the corresponding enum variant, falling back to `LogLevel::Debug` for
unrecognized input. Each field's `///` doc comment becomes part of the generated `--help` output.

> This example requires the [`derive`] feature flag to be enabled in `Cargo.toml`.

```rust,edition2024,no_run
use clap::Parser;
use std::net::IpAddr;

#[derive(Parser, Clone, Debug)]
enum LogLevel {
    Debug,
    Error,
    Warn,
    Trace,
    Info,
}

impl From<String> for LogLevel {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "debug" => LogLevel::Debug,
            "warn" => LogLevel::Warn,
            "error" => LogLevel::Error,
            "trace" => LogLevel::Trace,
            "info" => LogLevel::Info,
            _ => LogLevel::Debug, // The default value
        }
    }
}

#[derive(Parser, Debug)]
struct CliArgs {
    /// Logging threshold
    #[arg(short = 'l', long = "log-level")]
    log_level: Option<LogLevel>,

    /// The Backend Server Url
    #[arg(short = 'u', long = "server-url")]
    server_url: IpAddr,

    // The Backend Server Port
    #[arg(short = 'p', long = "server-port")]
    server_port: Option<u16>,

    /// Interval between database snapshots in seconds
    #[arg(short = 's', long = "snapshot-freq")]
    snapshot_frequency: Option<usize>,
}

fn main() {
    let args = CliArgs::parse();
    dbg!(args);
}
 ```

[`derive`]: https://docs.rs/clap/*/clap/_features/index.html
[`Parser`]: https://docs.rs/clap/*/clap/trait.Parser.html
