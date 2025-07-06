## Run an external command passing it stdin and check for an error code

[![std-badge]][std] [![cat-os-badge]][cat-os]

Opens the `python` interpreter using an external [`Command`] and passes it a
python statement for execution. [`Output`] of statement is then parsed.

```rust,edition2018,no_run
extern crate anyhow;
use anyhow::{Result, anyhow};
use std::collections::HashSet;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    let mut child = Command::new("python").stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    child.stdin
        .as_mut()
        .ok_or_else(|| anyhow!("Child process stdin has not been captured!"))?
        .write_all(b"import this; copyright(); credits(); exit()")?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout)?;
        let words = raw_output.split_whitespace()
            .map(|s| s.to_lowercase())
            .collect::<HashSet<_>>();
        println!("Found {} unique words:", words.len());
        println!("{:#?}", words);
        Ok(())
    } else {
        let err = String::from_utf8(output.stderr)?;
        return Err(anyhow!("External command failed:\n {}", err));
    }
}
```

[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`Output`]: https://doc.rust-lang.org/std/process/struct.Output.html
