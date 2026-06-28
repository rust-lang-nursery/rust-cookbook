# Clap Derive Parser Macro

Clap's [`Parser`] derive macro is the most common way to define a command line interface in Rust.
Instead of manually parsing `std::env::args()`, you describe the shape of your arguments as a
struct, and Clap generates all the parsing logic for you. This recipe defines the arguments for a
backend server, including a required server URL and several optional settings.

Whether an argument is required or optional depends on its type. Wrapping a field in `Option<T>`
makes it optional, while using `T` directly makes it required. In this recipe, `server_address` is
the only required argument, while `log_level`, `server_port`, and `snapshot_frequency` may all be
omitted when running the program.

Since command line input always arrives as plain text, custom types like `LogLevel` need a way to be
matched against text. Deriving [`ValueEnum`] tells Clap which strings map to which variants, by
default the lowercased variant names, so `--log-level warn` selects `LogLevel::Warn`. Any other
value is rejected before the program runs, with an error that lists the valid choices.  Each field's
`///` doc comment becomes part of the generated `--help` output.

> This recipe requires the [`derive`] feature flag to be enabled in `Cargo.toml`.

```rust,edition2024,no_run
use clap::{Parser, ValueEnum};
use std::net::IpAddr;

#[derive(ValueEnum, Clone, Debug)]
enum LogLevel {
    Debug,
    Error,
    Warn,
    Trace,
    Info,
}

#[derive(Parser, Clone, Debug)]
struct CliArgs {
    /// Logging threshold
    #[arg(short = 'l', long = "log-level")]
    log_level: Option<LogLevel>,

    /// The Backend Server Address
    #[arg(short = 'a', long = "server-address")]
    server_address: IpAddr,

    /// The Backend Server Port
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
[`Parser`]: https://docs.rs/clap/*/clap/trait.Parser.html
[`ValueEnum`]: https://docs.rs/clap/*/clap/trait.ValueEnum.html
[`derive`]: https://docs.rs/clap/*/clap/_features/index.html
