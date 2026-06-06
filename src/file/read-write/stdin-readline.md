## Read a line from stdin

[![std-badge]][std] [![cat-filesystem-badge]][cat-filesystem]

[`io::stdin`] returns a handle to the standard input stream. Calling
[`Stdin::lock`] acquires an exclusive lock and gives access to
[`BufRead::read_line`], which appends characters — including the trailing
newline — into the supplied `String` buffer. The lock is released when it
goes out of scope.

```rust,edition2021,no_run
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut line = String::new();

    // lock() gives exclusive BufRead access to stdin.
    stdin.lock().read_line(&mut line)?;

    // The buffer includes the trailing newline; trim before using.
    print!("you typed: {}", line.trim_end());
    Ok(())
}
```

[`BufRead::read_line`]: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line
[`io::stdin`]: https://doc.rust-lang.org/std/io/fn.stdin.html
[`Stdin::lock`]: https://doc.rust-lang.org/std/io/struct.Stdin.html#method.lock
