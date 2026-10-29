//! Tests on the book's own structure: every recipe is listed in its section's table, in the order
//! of the sidebar and of the recipe pages, and shows the same chips and feature banners. They read
//! `src/` the way mdBook does (expanding `{{#include}}`, numbering headings the same way), so they
//! need no build. The checks themselves are in `tests/`.

pub mod book;
pub mod features;
pub mod markdown;

use std::path::{Path, PathBuf};

pub fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
        .to_path_buf()
}

/// Fails the test with one line per problem, or passes if there are none.
pub fn assert_no_problems(problems: &[String]) {
    assert!(problems.is_empty(), "\n{}\n", problems.join("\n"));
}
