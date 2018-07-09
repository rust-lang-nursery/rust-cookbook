# A Rust Cookbook &emsp; [![Build Status travis]][travis]  [![Build Status appveyor]][appveyor]

[Build Status travis]: https://api.travis-ci.org/rust-lang-nursery/rust-cookbook.svg?branch=master
[travis]: https://travis-ci.org/rust-lang-nursery/rust-cookbook
[Build Status appveyor]: https://ci.appveyor.com/api/projects/status/k56hklb7puv7c4he?svg=true
[appveyor]: https://ci.appveyor.com/project/rust-lang-libs/rust-cookbook

**[Read it here]**.

This _Rust Cookbook_ is a collection of simple [Rust] examples that
demonstrate good practices to accomplish common programming tasks,
using the crates of the Rust ecosystem.

These examples are complete, and suitable for copying directly into
new cargo projects. They are tested and guaranteed to work.

## Read it offline

If you'd like to read it locally:

```bash
$ git clone https://github.com/rust-lang-nursery/rust-cookbook
$ cd rust-cookbook
$ cargo install mdbook --vers "0.1.8"
$ mdbook serve --open
```

The output can also be opened from the `book` subdirectory in your web browser.

```bash
$ xdg-open ./book/index.html # linux
$ start .\book\index.html    # windows
$ open ./book/index.html     # mac
```

[Read it here]: https://rust-lang-nursery.github.io/rust-cookbook
[Rust]: https://www.rust-lang.org/

## Contributing

This project is intended to be easy for new [Rust] programmers to
contribute to, and an easy way to get involved with the Rust
community. It needs and welcomes help.

For details see [CONTRIBUTING.md] on GitHub.

[CONTRIBUTING.md]: https://github.com/rust-lang-nursery/rust-cookbook/blob/master/CONTRIBUTING.md

## License [![CC0-badge]][CC0-deed]

Rust Cookbook is licensed under Creative Commons Zero v1.0 Universal License
([LICENSE-CC0](LICENSE-CC0) or https://creativecommons.org/publicdomain/zero/1.0/legalcode)

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Rust Cookbook by you, as defined in the CC0-1.0 license, shall be
[dedicated to the public domain][CC0-deed] and licensed as above, without any additional
terms or conditions.

[CC0-deed]: https://creativecommons.org/publicdomain/zero/1.0/deed.en
[CC0-badge]: https://mirrors.creativecommons.org/presskit/buttons/80x15/svg/cc-zero.svg
