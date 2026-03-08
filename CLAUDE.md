# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is the **Rust Cookbook** — an mdBook-based collection of practical Rust examples using ecosystem crates. Examples are written as markdown with embedded Rust code blocks, tested via [skeptic](https://github.com/brson/rust-skeptic) (which compiles and runs code blocks from markdown).

## Build & Test Commands

```bash
# Build the book
make build                    # or: mdbook build

# Run all tests (skeptic + spellcheck)
make test                     # or: cargo test && ./ci/spellcheck.sh list

# Run only skeptic tests (tests code examples in markdown)
cargo test --test skeptic

# Serve book locally with live reload
make serve                    # or: mdbook serve --open

# Build + test together
make dev

# xtask alternatives
cargo xtask test all          # run all tests
cargo xtask test cargo        # cargo tests only
cargo xtask test link         # link checking (requires lychee)
cargo xtask book              # build book
cargo xtask book serve        # serve book

# Link checking
cargo xtask test link         # or: lychee --base . --config ./ci/lychee.toml .

# Spellcheck
./ci/spellcheck.sh            # interactive; add words to ci/dictionary.txt
```

Required tools: `cargo install mdbook@0.4.43 lychee@0.17.0`

## Architecture

### Two example patterns

This project has two ways examples are tested. **Do not change dependency versions in root `Cargo.toml`** — if a dependency needs upgrading, migrate the affected examples to a standalone crate instead.

#### Pattern 1: Skeptic-tested inline code (legacy)

Code blocks live directly in markdown files under `src/`. The `build.rs` walks `src/` for `.md` files and `skeptic::generate_doc_tests()` compiles each code block as a test. All dependencies come from the root `Cargo.toml` `[dependencies]`.

Example (`src/text/regex/replace.md`):
~~~markdown
```rust,edition2018
use regex::Regex;
fn main() {
    // ...
}
```
~~~

- Annotations: `edition2018`, `edition2021`, `edition2024`, `no_run`, `should_panic`, `ignore`
- Hidden boilerplate lines prefixed with `# ` (visible to skeptic, hidden in book)
- Dependencies must exist in root `Cargo.toml` `[dependencies]`

#### Pattern 2: Standalone workspace crates (preferred for new/migrated examples)

Code lives in `crates/` as its own package with independent `Cargo.toml`. Markdown in `src/` pulls the code in via mdBook `{{#include}}` directives. The path is excluded from skeptic in `build.rs`.

Existing crates:
- `crates/algorithms/randomness/` — binaries in `src/bin/`, uses `workspace = true` deps
- `crates/development_tools/debugging/tracing/` — single binary
- `crates/web/` — library with modules, has its own pinned dependency versions

Example markdown (`src/algorithms/randomness/rand.md`):
~~~markdown
```rust
{{#include ../../../crates/algorithms/randomness/src/bin/rand.rs }}
```
~~~

Line-range includes are also supported (e.g., `{{#include .../passwd.rs::15 }}` to hide test code).

### Migrating a skeptic example to a standalone crate

When a dependency version needs to change, migrate the affected examples rather than bumping the version in root `Cargo.toml`. Steps:

1. **Create the crate** at `crates/<category>/<name>/` with its own `Cargo.toml`. Use `workspace = true` for shared deps when possible, or pin versions independently.
2. **Move code** from the markdown code block into `src/bin/<example>.rs` (one binary per example) or `src/lib.rs` for library-style examples.
3. **Add to workspace** in root `Cargo.toml`: add the path to `[workspace] members`.
4. **Update markdown** to use `{{#include}}` instead of inline code:
   ~~~markdown
   ```rust
   {{#include ../../../crates/<category>/<name>/src/bin/<example>.rs }}
   ```
   ~~~
   Use relative path from the markdown file's location to the crate source.
5. **Exclude from skeptic** in `build.rs`:
   - For an entire directory: add to `REMOVED_PREFIXES` (e.g., `"./src/<category>/<name>/"`)
   - For a single file: add to `REMOVED_TESTS` (e.g., `"./src/<category>/<name>/<file>.md"`)
6. **Test**: run `cargo test -p <crate-name>` for the new crate and `cargo test --test skeptic` to confirm nothing else broke.

### Key files

- `Cargo.toml` — root package with skeptic-tested dependencies and workspace member list
- `build.rs` — discovers markdown for skeptic; `REMOVED_TESTS` and `REMOVED_PREFIXES` exclude migrated examples
- `book.toml` — mdBook configuration
- `.cargo/config.toml` — defines `cargo xtask` alias
- `xtask/` — task runner for build, test, and link checking

## Writing Examples

- Examples should be complete, compilable, and use `?` for error handling (not `unwrap`)
- Avoid glob imports so readers can see which traits are used
- Mark examples needing external services with `no_run`
- Hide boilerplate with `# ` prefix (hidden from book, visible to skeptic)
- Prefer creating standalone crates for new examples over adding dependencies to root `Cargo.toml`
