# Contributing to the Rust Cookbook

The cookbook needs contributors and is intended to be easy to
contribute to. Help is welcome.

* [Building and testing](#building-and-testing)
* [Finding what to contribute](#finding-what-to-contribute)
* [Adding an example](#how-to-contribute)
* [Example guidelines](#example-guidelines)

## Building and testing

To start, clone the cookbook from git and navigate to that directory:

```
git clone https://github.com/brson/rust-cookbook.git
cd rust-cookbook
```

Cookbook is built with [mdbook], so install that first with cargo:

```
cargo install mdbook
```

And then build:

```
mdbook build
```

The output will be in the `book/` directory. You can open it from
there in a web browser.

All examples in the cookbook are tested with [skeptic], a tool for
testing arbitrary markdown documentation in a style similar to
rustdoc.

To run the cookbook test suite:

```
cargo test
```

[mdbook]: http://azerupi.github.io/mdBook/index.html
[skeptic]: https://github.com/brson/rust-skeptic

## Finding what to contribute

This project is intended to be simple to contribute to, and to always
have obvious next work items available. If at any time there is not
something obvious to contribute, that is a bug. Please ask for
assistance on the [libz blitz] thread, or email Brian Anderson
directly (banderson@mozilla.com).

The development process for the cookbook is presently oriented around
crates: we decide which crates to represent in the cookbook, then come
up with example use cases to write, then write the examples. And those
are the three basic, recurring types of contributions needed.

The development process for the cookbook today is tied to the [libz
blitz], a broader project to improve the Rust crate ecosystem, and the
cookbook presently represents the crates under consideration there.
The easiest way to find the most immediate work needed for the
cookbook is to follow the "What's next" section at the top of that
thread, which should at all times link to something to contribute to
the cookbook.

Otherwise, look on the issue tracker for the [crate-tasks] tag. These
contain checklists of the examples we want to provide for individual
crates. The simplest way to contribute is to claim one of these
examples, and submit a PR adding it. If you do claim one, please leave
a comment saying so, so others don't accidentally duplicate your work.

If you have an idea for an example for a specific crate, please
suggest it on those issues.

Please do not submit examples for crates not yet represented in the
coobook, unless it is part of the libz blitz crate schedule.
Contribution will be open to a broader set of crates in the future.
For more about which crates are represented in the cookbook, see ["a
note about crate representation"][which-crates] in the cookbok.

[crate-tasks]: https://github.com/brson/rust-cookbook/issues?q=is%3Aissue+is%3Aopen+label%3Acrate-tasks
[which-crates]: https://brson.github.io/rust-cookbook/about.html#which-crates
[libz blitz]: https://internals.rust-lang.org/t/rust-libz-blitz/5184

## Adding an example

Adding an example involves:

- Deciding which _section_ of the book it belongs in
- Deciding which _categories_ apply to it
- Adding the example to a the section index in intro.md
- Adding the example to the appropriate section markdown file
- Updating 'badges' and hyperlinks as needed
- Writing a useful description of the example

Examples are presently organized in three ways:

- Book sections - the cookbook is a book, and is organized like a book
  in logical sections, like "basics", "serialization", "concurrency".
- Category tags - each example is tagged with one or more category
  tags, like "math", "file-io".
- Crate tags - each example is tagged with one or more crate tags,
  indicating which crates are represented in the example. Those that
  use no additional crates are simply tagged 'std'.

For more about the organization of the book see ["how to read this
book"] in the cookbook.

Hopefully your example belongs to an obvious section and categories,
but since the cookbook is so new, quite possibly not. Ask on thread.

For most steps you can simply follow the lead of existing examples.
The art comes in writing effective examples.

## Example guidelines

Examples in the cookbook have these goals and qualities:

- They can described by a single sentence that states
  their utility
- They can be read and understood by complete beginners
- They are standalone examples that can be copied into a learners'
  own workspace and compiled and modified for experimentation
- They demonstrate real tasks, such that experienced developers
  can use as a reference
- They follow best practices and do not take shortcuts
- They use consistent error handling

Examples should have a simple single-sentence title that describes
something a typical Rust user typically wants to do.

Example are intended to be read by complete beginners, and copied into
projects for experimentation. They should follow best practices and
not take shortcuts.

The example should have minimal code that doesn't directly support the
description of the example. Keep extra functions and types to a
minimum.

Follow the error handling templates in ["A note about error
handling"][errors]. Examples always set up error handling correctly and
propagate errors with `?` (not `try!`).

Don't use glob imports, even for preludes, so that users can see what
traits they are calling. (Some might consider using glob imports for
preludes a best practice, making this awkward).

Sort imports.

Examples should be simple and obvious enough that an experienced dev
won't need comments. Describe the code in prose, not in
comments. Things that should be described include traits imported and
their methods used. Think about what information here supports the use
case and might not be obvious to someone new. Say the minimum possible
about aspects that don't directly support the use case. See
["basics"] for examples.

["basics"]: https://brson.github.io/rust-cookbook/basics.html

Hyperlink all references to APIs, either on doc.rust-lang.org/std or
docs.rs, and style them as `code`.

Finally, this book is intended to also demonstrate the integration
of crates that work well together. Super bonus points for examples
that feature multiple crates sensibly.

[errors]: https://brson.github.io/rust-cookbook/about.html#errors

