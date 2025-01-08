# xtask - (Rust Cookbook)

**Rust Dependencies**:
   - Make sure you have the required tools installed:
     ```bash
     cargo install mdbook@0.4.43 lychee@0.17.0
     ```

## Available Tasks

### `test`
Run various tests for the project. You can specify individual tests or run them all.

- `cargo`: Run the `cargo test` command for the Rust code.
- `spellcheck`: Run the spellcheck script.
- `link`: Verify links within the project.
- `all`: Run all the tests (default).

**Usage:**
```bash
cargo xtask test [all|cargo|spellcheck|link]
```

### `book`
Build or serve the project's documentation using `mdbook`.

- `build`: Build the book (default).
- `serve`: Serve the book locally and open it in a browser.

**Usage:**
```bash
cargo xtask book [build|serve]
```
