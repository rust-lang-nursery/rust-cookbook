# A Rust Cookbook

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
$ cargo install mdbook --vers "0.4.43"
$ mdbook serve --open
```

The output can also be opened from the `book` subdirectory in your web browser.

```bash
$ xdg-open ./book/index.html # linux
$ start .\book\index.html    # windows
$ open ./book/index.html     # mac
```

## Development

### Local Development

For local development and testing, you can use the provided Makefile:

```bash
# Show all available commands
make help

# Build the book locally
make build

# Run tests
make test

# Build and test (development workflow)
make dev

# Serve the book locally with live reload
make serve

# Clean build artifacts
make clean
```

### Deployment

As a maintainer, you can deploy the site locally using:

```bash
# Deploy to GitHub Pages (requires maintainer permissions)
make deploy

# Or use the script directly
./scripts/deploy.sh
```

The deployment script will:
1. Build and test the book
2. Push the built site to the `gh-pages` branch
3. GitHub Pages will automatically serve the updated site

**Note**: This requires maintainer permissions to push to the `gh-pages` branch.

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
([LICENSE-CC0](LICENSE-CC0) or https://creativecommons.org/)

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Rust Cookbook by you, as defined in the CC0-1.0 license, shall be
[dedicated to the public domain][CC0-deed] and licensed as above, without any additional
terms or conditions.

[CC0-deed]: https://creativecommons.org/publicdomain/zero/1.0/
[CC0-badge]: https://mirrors.creativecommons.org/presskit/buttons/80x15/svg/cc-zero.svg
