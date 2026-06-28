# Clap Subcommands

Many command line tools split their functionality into subcommands. Familiar tool commands include
`git commit` and `cargo build`, where the first word selects a specific action with its own set of
arguments. Clap supports this pattern through the [`Subcommand`] trait, shown here in a small
note-taking application with four subcommands: `new`, `list`, `delete`, and `update`.

Each variant of the `Commands` enum represents one subcommand, and the fields on that variant become
the arguments accepted by it. By default, these fields are parsed as positional arguments. Adding
`#[arg(short, long)]` turns a field into a named flag or option instead, such as `--title` or `-t`.
Doc comments written above each variant and field are also used by Clap to automatically generate
the help text shown when running the program with `--help`.

Some arguments are not specific to a single subcommand. The `color` field on NotesArgs is marked
with ``#[arg(global = true)]``, which makes it available before or after any subcommand, such as
`notes --color new` or `notes new --color`. This is a convenient way to define flags or options that
should apply across the entire application rather than to one action.

> This recipe requires the [`derive`] feature flag to be enabled in `Cargo.toml`.

```rust,edition2024,no_run
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct NotesArgs {
    /// Enable syntax highlighting
    #[arg(global = true, short = 'c', long = "color")]
    color: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new note
    New {
        #[arg(short = 't', long = "title", help = "Title of the new note")]
        title: String,
        #[arg(short = 'b', long = "body", help = "Description of the note")]
        body: Option<String>,
    },

    /// List all available notes
    List,

    /// Delete a note by its index. 1-based
    Delete {
        /// Index of the note
        index: u64,
    },

    /// Update an existing note
    Update {
        /// Index of the note
        index: u64,

        #[arg(short = 't', long = "title", help = "Updated title of the note")]
        title: Option<String>,

        #[arg(short = 'b', long = "body", help = "Updated description of the note")]
        body: Option<String>,
    },
}

fn main() {
    let args = NotesArgs::parse();
    if args.color {
        println!("Enabled syntax highlighting");
    }

    match args.command {
        Commands::New { title, body } => {
            // Prints:
            //      TITLE: ...
            //      BODY: ...
            println!(
                "Creating new note!\nTITLE: {}\n{}",
                title,
                if let Some(body) = body {
                    format!("BODY: {}", body)
                } else {
                    "".into()
                }
            );
        }
        Commands::List => println!("Listing available tasks"),
        Commands::Delete { index } => println!("Deleting note {index}"),
        Commands::Update { index, title, body } => {
            println!("Updating note {index}");
            if let Some(title) = title {
                println!("NEW TITLE: {title}");
            }
            if let Some(body) = body {
                println!("NEW BODY: {body}");
            }
        }
    }
}
```

[`derive`]: https://docs.rs/clap/*/clap/_features/index.html
[`Subcommand`]: https://docs.rs/clap/*/clap/trait.Subcommand.html
