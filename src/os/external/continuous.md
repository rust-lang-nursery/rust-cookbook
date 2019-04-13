## Continuously process child process' outputs

[![std-badge]][std] [![cat-os-badge]][cat-os]

In [Run an external command and process stdout](#run-an-external-command-and-process-stdout),
processing doesn't start until external [`Command`] is finished.
The recipe below calls [`Stdio::piped`] to create a pipe, and reads
`stdout` continuously as soon as the [`BufReader`] is updated.

The below recipe is equivalent to the Unix shell command
`journalctl | grep usb`.

```rust,no_run
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn main() -> Result<(), Error> {
    let stdout = Command::new("journalctl")
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output."))?;

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.find("usb").is_some())
        .for_each(|line| println!("{}", line));

     Ok(())
}
```

[`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`Stdio::piped`]: https://doc.rust-lang.org/std/process/struct.Stdio.html
