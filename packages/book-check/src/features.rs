//! Which Cargo features the code of a recipe needs, and where the recipe says so.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

/// The dependencies a recipe uses without needing the features its crate enables for other recipes,
/// from a line such as `<!-- needs-no-features: ndarray, nalgebra -->` anywhere in its markdown.
pub fn needs_no_features(markdown: &str) -> HashSet<String> {
    markdown
        .lines()
        .filter_map(|line| {
            line.trim()
                .strip_prefix("<!-- needs-no-features:")?
                .strip_suffix("-->")
        })
        .flat_map(|names| names.split(',').map(|n| n.trim().to_string()))
        .collect()
}

/// Whether `text` uses `ident` as a name of its own (`serde::` yes, `serde_json::` no).
pub fn mentions(text: &str, ident: &str) -> bool {
    let word = |c: char| c.is_alphanumeric() || c == '_';
    text.match_indices(ident).any(|(at, _)| {
        !text[..at].chars().next_back().is_some_and(word)
            && !text[at + ident.len()..].chars().next().is_some_and(word)
    })
}

/// The dependencies of a standalone crate that enable features, as `name -> features`.
pub fn dependencies_with_features(manifest: &Path) -> HashMap<String, BTreeSet<String>> {
    let text = fs::read_to_string(manifest).unwrap();
    let table: toml::Table = text
        .parse()
        .unwrap_or_else(|e| panic!("{}: {e}", manifest.display()));
    let mut tables = Vec::new();
    if let Some(t) = table.get("dependencies").and_then(|t| t.as_table()) {
        tables.push(t);
    }
    for target in table
        .get("target")
        .and_then(|t| t.as_table())
        .into_iter()
        .flatten()
    {
        if let Some(t) = target.1.get("dependencies").and_then(|t| t.as_table()) {
            tables.push(t);
        }
    }
    let mut out = HashMap::new();
    for (name, dep) in tables.into_iter().flatten() {
        let features: BTreeSet<String> = dep
            .get("features")
            .and_then(|f| f.as_array())
            .into_iter()
            .flatten()
            .filter_map(|f| f.as_str().map(str::to_string))
            .collect();
        if !features.is_empty() {
            out.insert(name.clone(), features);
        }
    }
    out
}

/// The `{{#include}}`d source files of a markdown file that live in a crate under `crates/`, as
/// `(crate directory, source text)`. Line ranges (`file.rs::71`) are ignored.
pub fn included_crate_sources(page: &Path) -> Vec<(PathBuf, String)> {
    let text = fs::read_to_string(page).unwrap();
    let mut out = Vec::new();
    for line in text.lines() {
        let Some(file) = line
            .trim()
            .strip_prefix("{{#include ")
            .and_then(|l| l.strip_suffix("}}"))
        else {
            continue;
        };
        let file = file.trim().split(':').next().unwrap_or("");
        if file.ends_with(".md") || !file.contains("crates/") {
            continue;
        }
        let path = page.parent().unwrap().join(file);
        let Ok(source) = fs::read_to_string(&path) else {
            continue;
        };
        let mut dir = path.parent();
        while let Some(d) = dir {
            if d.join("Cargo.toml").exists() {
                out.push((d.to_path_buf(), source));
                break;
            }
            dir = d.parent();
        }
    }
    out
}

/// For each markdown file, the pages that `{{#include}}` it.
pub fn including_pages(files: &[PathBuf]) -> HashMap<PathBuf, Vec<PathBuf>> {
    let mut parents: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
    for page in files {
        for line in fs::read_to_string(page).unwrap().lines() {
            let included = line
                .trim()
                .strip_prefix("{{#include ")
                .and_then(|l| l.strip_suffix("}}"))
                .map(str::trim)
                .filter(|f| f.ends_with(".md"));
            if let Some(file) = included {
                let path = page.parent().unwrap().join(file);
                if let Ok(path) = path.canonicalize() {
                    parents.entry(path).or_default().push(page.clone());
                }
            }
        }
    }
    parents
}
