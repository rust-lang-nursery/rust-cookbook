## Format text into a String with write!

[![std-badge]][std] [![cat-filesystem-badge]][cat-filesystem]

[`String`] implements [`fmt::Write`], so the [`write!`] macro can push
formatted text into it with no I/O and no file handle. Import
[`fmt::Write`] — not [`io::Write`] — to bring the required trait into scope;
the two traits share the macro name but operate on different targets.
The `String` implementation never actually errors, but the signature uses
`?` for consistency with other writers.

```rust,edition2021
use std::fmt::Write;

fn main() -> std::fmt::Result {
    let mut s = String::new();

    // write! on a String uses fmt::Write — no I/O, no file handle needed.
    write!(s, "Hello, {}!", "world")?;
    write!(s, " The answer is {}.", 42)?;

    assert_eq!(s, "Hello, world! The answer is 42.");
    println!("{s}");
    Ok(())
}
```

[`fmt::Write`]: https://doc.rust-lang.org/std/fmt/trait.Write.html
[`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
[`write!`]: https://doc.rust-lang.org/std/macro.write.html
