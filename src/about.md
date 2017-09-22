# About "Cookin' with Rust"

## Table of contents

- [Who this book is for](#who-this-book-is-for)
- [How to read this book](#how-to-read-this-book)
- [How to use the recipes](#how-to-use-the-recipes)
- [A note about error handling](#a-note-about-error-handling)
- [A note about crate representation](#a-note-about-crate-representation)

## Who this book is for

This cookbook is intended for new Rust programmers, so that they may
quickly get an overview of the capabilities of the Rust crate
ecosystem. It is also intended for experienced Rust programmers, who
should find in the recipes an easy reminder of how to accomplish
common tasks.

## How to read this book

The cookbook [index] contains the full list of recipes, organized into
a number of sections: "basics", "encoding", "concurrency", etc.  The
sections themselves are more or less ordered in progression, with
later sections being more advanced, and occasionally building on
concepts from earlier sections.

Within the index, each section contains a list of recipes. The recipes
are simple statements of a task to accomplish, like "generate random
numbers in a range"; and each recipe is tagged with badges indicating
which _crates_ they use, like [![rand-badge]][rand], and which
categories on [crates.io] those crates belong to, like
[![cat-science-badge]][cat-science].

New Rust programmers should be comfortable reading from the first
section to the last, and doing so should give one a strong overview of
the crate ecosystem. Click on the section header in the index, or in
the sidebar to navigate to the page for that section of the book.

If you are simply looking for the solution to a simple task, the
cookbook is today more difficult to navigate. The easiest way to find
a specific recipe is to scan the index looking for the crates and
categories one is interested in. From there, click on the name of the
recipe to view it. This will improve in the future.

## How to use the recipes

Recipes are designed to give you instant access to working code, along
with a full explanation of what it is doing, and to guide you to
further information.

All recipes in the cookbook are full, self contained programs, so
that they may be copied directly into your own projects for
experimentation. To do so follow the instructions below.

Consider this example for "generate random numbers within a range":

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

```rust
extern crate rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Random f64: {}", rng.gen::<f64>());
}
```

To work with it locally we can run the following commands to create
a new cargo project, and change to that directory:


```sh
cargo new my-example --bin
cd my-example
```

Now, we also need to add the necessary crates to [Cargo.toml], as
indicated by the crate badges, in this case just "rand". To do so,
we'll use the `cargo add` command, which is provided by the
[`cargo-edit`] crate, which we need to install first:

```sh
cargo install cargo-edit
cargo add rand
```

Now you can replace `src/main.rs` with the full contents of the
example and run it:

```sh
cargo run
```

The crate badges that accompany the examples link to the crates' full
documentation on [docs.rs], and is often the next documentation you
should read after deciding which crate suites your purpose.

## A note about error handling

Error handling in Rust is robust when done correctly, but in today's
Rust it requires a fair bit of boilerplate. Because of this one often
sees Rust examples filled with `unwrap` calls instead of proper error
handling.

Since these recipes are intended to be reused as-is and encourage best
practices, they set up error handling correctly when there are
`Result` types involved.

The basic pattern we use is to have a `fn run() -> Result` that acts
like the "real" main function. We use the [error-chain] crate to make
`?` work within `run`.

The structure generally looks like:

```rust
#[macro_use]
extern crate error_chain;

use std::net::IpAddr;
use std::str;

error_chain! {
    foreign_links {
        Utf8(std::str::Utf8Error);
        AddrParse(std::net::AddrParseError);
    }
}

fn run() -> Result<()> {
    let bytes = b"2001:db8::1";

    // Bytes to string.
    let s = str::from_utf8(bytes)?;

    // String to IP address.
    let addr: IpAddr = s.parse()?;

    println!("{:?}", addr);
    Ok(())
}

quick_main!(run);
```

This is using the `error_chain!` macro to define a custom `Error` and
`Result` type, along with automatic conversions from two standard
library error types. The automatic conversions make the `?` operator
work. The `quick_main!` macro generates the actual `main` function and
prints out the error if one occurred.

For the sake of readability error handling boilerplate is hidden by
default like below.  In order to read full contents click on the
"expand" (<i class="fa fa-expand"></i>) button located in the top
right corner of the snippet.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate url;

use url::{Url, Position};
#
# error_chain! {
#     foreign_links {
#         UrlParse(url::ParseError);
#     }
# }

fn run() -> Result<()> {
    let parsed = Url::parse("https://httpbin.org/cookies/set?k2=v2&k1=v1")?;
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("cleaned: {}", cleaned);
    Ok(())
}
#
# quick_main!(run);
```

For more background on error handling in Rust, read [this page of the
Rust book][error-docs] and [this blog post][error-blog].

## A note about crate representation

This cookbook is intended eventually to provide expansive coverage of
the Rust crate ecosystem, but today is limited in scope while we get
it bootstrapped and work on the presentation. Hopefully, starting
from a small scope and slowly expanding will help the cookbook become
a high-quality resource sooner, and allow it to maintain consistent
quality levels as it grows.

At present the cookbook is focused on the standard library, and on
"core", or "foundational", crates—those crates that make up the most
common programming tasks, and that the rest of the ecosystem builds
off of.

The cookbook is closely tied to the [Rust Libz Blitz], a project to
identify, and improve the quality of such crates, and so it largely
defers crate selection to that project. Any crates that have already
been evaluated as part of that process are in scope for the cookbook,
as are crates that are pending evaluation.

{{#include links.md}}

[index]: intro.html
[error-docs]: https://doc.rust-lang.org/book/error-handling.html
[error-blog]: https://brson.github.io/2016/11/30/starting-with-error-chain
[error-chain]: https://docs.rs/error-chain/
[Rust Libz Blitz]: https://internals.rust-lang.org/t/rust-libz-blitz/5184
[crates.io]: https://crates.io
[docs.rs]: https://docs.rs
[Cargo.toml]: http://doc.crates.io/manifest.html
[`cargo-edit`]: https://github.com/killercup/cargo-edit
