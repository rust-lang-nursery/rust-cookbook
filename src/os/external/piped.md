## Run piped external commands

[![std-badge]][std] [![cat-os-badge]][cat-os]

Shows up to the 10<sup>th</sup> biggest files and subdirectories in
the current working directory. It is equivalent to running: `du -ah . |
sort -hr | head -n 10`.

[`Command`]s represent a process. Output of a child process is captured with a
[`Stdio::piped`] between parent and child.

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

[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`Stdio::piped`]: https://doc.rust-lang.org/std/process/struct.Stdio.html
