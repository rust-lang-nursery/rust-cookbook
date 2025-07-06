## Run an external command and process stdout

[![std-badge]][std] [![cat-os-badge]][cat-os]

Runs `git log --oneline` using an external [`Command`] and inspects the [`Output`]
status to determine if the command was successful. The command output is captured
as a [`String`] using [`String::from_utf8`].

```rust,edition2018,no_run
extern crate anyhow;
use anyhow::{Result, anyhow};
use std::process::Command;

fn main() -> Result<()> {
    let output = Command::new("git").arg("log").arg("--oneline").output()?;

    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout)?;
        let lines = raw_output.lines();
        println!("Found {} lines", lines.count());
        Ok(())
    } else {
        return Err(anyhow!("Command executed with failing error code"));
    }
}
```

[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`Output`]: https://doc.rust-lang.org/std/process/struct.Output.html
[`Regex`]: https://docs.rs/regex/*/regex/struct.Regex.html
