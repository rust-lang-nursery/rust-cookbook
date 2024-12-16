mod tests;
mod mdbook;

use std::path::{Path, PathBuf};
use std::{env, error::Error};

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), Box<dyn Error>> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("test") => {
            let sub_task = env::args().nth(2).unwrap_or_else(|| "all".to_string());
            tests::run_test(&sub_task)?
        }
        Some("book") => {
            let sub_task = env::args().nth(2).unwrap_or_else(|| "build".to_string());
            mdbook::run_book(&sub_task)?
        }
        _ => print_help(),
    }
    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn print_help() {
    eprintln!("Available tasks:");
    eprintln!(
        "  test [all|cargo|spellcheck|link]    - Run the tests. Use 'all' to run all tests (default), or specify individual tests."
    );
    eprintln!(
        "  book [build]  - Build the book using mdbook. Default if no subcommand is specified."
    );
    eprintln!("  book serve    - Serve the book using mdbook and open it in a browser.");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  cargo xtask <task> [subcommand]");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  cargo xtask test");
    eprintln!("  cargo xtask test all");
    eprintln!("  cargo xtask test cargo");
    eprintln!("  cargo xtask book");
    eprintln!("  cargo xtask book serve");
}