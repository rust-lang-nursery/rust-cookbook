## Basic Argument Parsing
This section covers a basic implementation of commandline argument parsing.
Use [`args`] to get the arguments this process was started with:

```rust,edition2024
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(!args.is_empty());

    // Skip the program name, it's usually the first argument
    let args: Vec<String> = env::args().skip(1).collect();
}
```

Some args, e.g. file paths, may not be valid UTF-8. Use [`args_os`] to handle those safely:

```rust,edition2024
use std::{env, ffi::OsString};

fn main() {
    let os_args: Vec<OsString> = env::args_os().collect();
    assert!(!os_args.is_empty());
}
```

## Calculate File Stats
In this section we parse CLI args and compute per-file stats (line/word counts). A mini [`wc`]-like
utility

Flags:
- --lines, -l Count lines per file
- --words, -w Count words per file
- --paths, -p Remaining args treated as file paths

> `-p` acts as a terminator. Everything after it is consumed as file paths and parsing stops
> immediately.

`CliArgs` holds the parsed state of the process args. It uses [`args_os`] over [`args`] to
safely handle non-UTF-8 paths, which can occur on Linux. [`skip(1)`] drops the bin name since it is
always the first arg. Once `-p` flag is is encountered, all remaining args are collected into paths.

`Stats<'p>` represents the computed result for a single file. It borrows filepath directly from
`CliArgs` rather than cloning it, with the lifetime 'p tying each Stats instance to its source
`CliArgs`. Both `lines` and `words` are `Option<usize>`, where `None` means the flag was absent and
the stat was never computed, while `Some(n)` means the flag was set and the count is `n`. This
distinction matters because it avoids confusing "not requested" with a legitimate count of zero.

`stat_files` is the core logic. Iterates over `cli_args.paths` and builds a `Vec<Stats<'_>>`.
Non-files and IO errors are skipped silently via `continue` for simplicity.  Each file is read
to a string buffer with fs::[`read_to_string()`] for processing. Line count uses
content.[`lines()`].[`count()`]. Word count splits each line on whitespace via
[`split_whitespace()`], counts tokens per line, then sums across all lines, which correctly handles
tabs and multiple consecutive spaces. Both stats are computed only if their respective flag was set,
so there is no wasted work.

The result is output as one line per file. Stats omitted by the user are excluded from the output
entirely rather than printed as zero or a placeholder.

```rust,edition2024,no_run
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

// Represents the arguments passed to this process during initialization
#[derive(Debug)]
struct CliArgs {
    lines: bool,
    words: bool,
    paths: Vec<PathBuf>,
}

impl CliArgs {
    fn parse() -> Self {
        let mut lines = false;
        let mut words = false;
        let mut paths = Vec::new();

        let args = env::args_os().skip(1);
        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            match arg.to_str().unwrap_or_default() {
                "-l" | "--lines" => lines = true,
                "-w" | "--words" => words = true,
                "-p" | "--paths" => {
                    paths.extend(args.by_ref().map(PathBuf::from));
                    break;
                }
                _ => {}
            }
        }

        CliArgs {
            lines,
            words,
            paths,
        }
    }
}

// Represents the statistics of a given filepath
struct Stats<'p> {
    filepath: &'p Path,
    lines: Option<usize>,
    words: Option<usize>,
}

fn stat_files(cli_args: &CliArgs) -> Vec<Stats<'_>> {
    let mut stat_collection: Vec<Stats> = Vec::new();
    // If no flag is set, do no work
    if !cli_args.words && !cli_args.lines {
        return stat_collection;
    }

    for filepath in &cli_args.paths {
        // Skip anything that ain't a file
        if !filepath.is_file() {
            continue;
        }

        let Ok(content) = fs::read_to_string(filepath) else {
            continue;
        };

        let lines = match cli_args.lines {
            true => Some(content.lines().count()),
            false => None,
        };

        let words = match cli_args.words {
            true => Some(
                content
                    .lines()
                    .map(|line| line.split_whitespace().count())
                    .sum(),
            ),
            false => None,
        };

        let stats = Stats {
            filepath,
            lines,
            words,
        };

        stat_collection.push(stats);
    }

    stat_collection
}

fn main() {
    let cli_args = CliArgs::parse();
    let stats = stat_files(&cli_args);
    // Print individual file statistics
    for stat in stats {
        println!(
            "Path: {:?} {} {}",
            stat.filepath,
            if let Some(words) = stat.words {
                format!("Words: {}", words)
            } else {
                "".to_string()
            },
            if let Some(lines) = stat.lines {
                format!("Lines: {}", lines)
            } else {
                "".to_string()
            },
        );
    }
}
```

Example Usage:

```bash
cargo run --release -- -l -w -p src/main.rs Cargo.toml
# Path: "src/main.rs" Words: 312 Lines: 84
# Path: "Cargo.toml" Words: 21 Lines: 9

cargo run --release -- -l -p src/main.rs
# Path: "src/main.rs" Lines: 84
```

[`args_os`]: https://doc.rust-lang.org/std/env/fn.args_os.html
[`args`]: https://doc.rust-lang.org/std/env/fn.args.html
[`count()`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count
[`env`]: https://doc.rust-lang.org/std/env/index.html
[`lines()`]: https://doc.rust-lang.org/std/primitive.str.html#method.lines
[`read_to_string()`]: https://doc.rust-lang.org/std/fs/fn.read_to_string.html
[`skip(1)`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.skip
[`split_whitespace()`]: https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace
[`wc`]: https://www.man7.org/linux/man-pages/man1/wc.1.html
