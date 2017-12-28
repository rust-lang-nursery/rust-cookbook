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
$ cargo install mdbook --vers "0.0.28"
$ mdbook build
```

The output will be in the `book` subdirectory. Open it in your web browser.

_Linux:_
```bash
$ firefox book/index.html
$ google-chrome book/index.html
```

_OS X:_
```bash
$ open -a "Firefox" book/index.html
$ open -a "Google Chrome" book/index.html
```

_Windows:_
```bash
$ explorer book\index.html                             # Default Browser (PowerShell and Cmd)
$ Start-Process "firefox.exe" book\index.html          # PowerShell
$ start firefox.exe book\index.html                    # Cmd
$ Start-Process "chrome.exe" "\\$pwd\book\index.html"  # PowerShell
$ start chrome.exe "\\%cd%\book\index.html"            # Cmd
```


[Read it here]: https://rust-lang-nursery.github.io/rust-cookbook
[Rust]: https://www.rust-lang.org/

## Contributing

This project is intended to be easy for new [Rust] programmers to
contribute to, and an easy way to get involved with the Rust
community. It needs and welcomes help.

For details see [CONTRIBUTING.md] on GitHub.

[CONTRIBUTING.md]: https://github.com/rust-lang-nursery/rust-cookbook/blob/master/CONTRIBUTING.md

## License

Rust Cookbook is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Rust Cookbook by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
