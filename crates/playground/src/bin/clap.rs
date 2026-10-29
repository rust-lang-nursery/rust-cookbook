use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Who to greet
    #[arg(short, long, default_value = "world")]
    name: String,
    /// How many times
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    // Arguments are fixed here because the playground has none; use `Cli::parse()` in a real binary.
    let cli = Cli::parse_from(["demo", "--name", "Ferris", "--count", "2"]);
    for _ in 0..cli.count {
        println!("Hello, {}!", cli.name);
    }
}
