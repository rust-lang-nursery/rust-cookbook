//! mdBook preprocessor (`[preprocessor.chips]` in book.toml). Replaces the shields.io badges in the
//! section tables with crate and category pills, and writes `src/cookbook-index.js` for the
//! search box and crate index. Versions, features and repositories come from the `Cargo.toml`
//! files the recipes are tested against, so the pills need no network access in the browser.

use crate::project_root;
use crate::search_index;
use serde_json::Value;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{error::Error, fs};

const STD_REPO: &str = "https://github.com/rust-lang/rust";

const COPY_ICON: &str = r#"<svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path fill="currentColor" d="M4 2a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v7a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V2zm2-.5a.5.5 0 0 0-.5.5v7a.5.5 0 0 0 .5.5h6a.5.5 0 0 0 .5-.5V2a.5.5 0 0 0-.5-.5H6zM2 5a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2v-1H8.5v1a.5.5 0 0 1-.5.5H2a.5.5 0 0 1-.5-.5V7A.5.5 0 0 1 2 6.5h1V5H2z"/></svg>"#;
const GITHUB_ICON: &str = r#"<svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path fill="currentColor" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82a7.6 7.6 0 0 1 4 0c1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8z"/></svg>"#;

/// What the pill for one crate offers: a `Cargo.toml` line to copy and a repository link.
#[derive(Default, Clone)]
pub struct CrateMeta {
    pub snippet: Option<String>,
    pub repo: Option<String>,
}

pub struct Meta(HashMap<String, CrateMeta>);

fn normalize(name: &str) -> String {
    name.to_lowercase().replace('-', "_")
}

impl Meta {
    pub fn get(&self, name: &str) -> CrateMeta {
        if name == "std" {
            return CrateMeta {
                snippet: None,
                repo: Some(STD_REPO.to_string()),
            };
        }
        self.0.get(&normalize(name)).cloned().unwrap_or_default()
    }

    /// Reads every manifest in the repo through `cargo metadata`. The root package comes first so
    /// its requirements (the ones the inline, skeptic-tested snippets compile against) win over a
    /// standalone crate that pins something else.
    ///
    /// The result is cached in `target/mdbook-chips.json` and reused while no `Cargo.toml` or
    /// `Cargo.lock` in the repo has changed, which saves the several `cargo metadata` calls.
    pub fn load() -> Result<Meta, Box<dyn Error>> {
        let root = project_root();
        let cache = root.join("target").join("mdbook-chips.json");
        let key = fingerprint(&root);
        if let Some(meta) = read_cache(&cache, &key) {
            return Ok(meta);
        }
        let (meta, complete) = Meta::resolve(&root)?;
        if complete {
            write_cache(&cache, &key, &meta);
        }
        Ok(meta)
    }

    fn resolve(root: &Path) -> Result<(Meta, bool), Box<dyn Error>> {
        let mut complete = true;
        // Run one at a time: parallel `cargo metadata` calls fight over cargo's package-cache lock
        // and end up several times slower.
        let mut stray = Vec::new();
        find_manifests(root, &mut stray);
        stray.retain(|p| p.starts_with(root.join("crates")) && p.ends_with("Cargo.toml"));
        stray.sort();
        let main = cargo_metadata(&root.join("Cargo.toml"));
        let main = main?;
        let known: Vec<String> = members(&main).map(|p| str_of(p, "manifest_path")).collect();

        // Crates in the workspace's `exclude` list are not in its metadata; they get their own.
        let mut manifests: Vec<Value> = vec![main.clone()];
        for path in stray {
            if known.iter().any(|k| Path::new(k) == path) {
                continue;
            }
            match cargo_metadata(&path) {
                Ok(m) => manifests.push(m),
                Err(e) => {
                    eprintln!("warning: skipping {}: {e}", path.display());
                    complete = false;
                }
            }
        }

        let mut repos: HashMap<String, String> = HashMap::new();
        let mut out: HashMap<String, CrateMeta> = HashMap::new();
        for metadata in &manifests {
            for package in metadata["packages"].as_array().into_iter().flatten() {
                if let Some(repo) = package["repository"].as_str().and_then(clean_repo) {
                    repos.insert(normalize(&str_of(package, "name")), repo);
                }
            }
        }
        for metadata in &manifests {
            let mut packages: Vec<&Value> = members(metadata).collect();
            packages.sort_by_key(|p| {
                (
                    str_of(p, "name") != "rust-cookbook",
                    str_of(p, "manifest_path"),
                )
            });
            for package in packages {
                for dep in package["dependencies"].as_array().into_iter().flatten() {
                    // Path dependencies are the cookbook's own crates.
                    if dep["source"].is_null() {
                        continue;
                    }
                    let name = str_of(dep, "name");
                    out.entry(normalize(&name)).or_insert_with(|| CrateMeta {
                        snippet: Some(snippet(&name, dep)),
                        repo: None,
                    });
                }
            }
        }
        for (name, meta) in &mut out {
            meta.repo = repos.get(name).cloned();
        }
        Ok((Meta(out), complete))
    }
}

fn str_of(v: &Value, key: &str) -> String {
    v[key].as_str().unwrap_or("").to_string()
}

fn members(metadata: &Value) -> impl Iterator<Item = &Value> {
    let ids: Vec<&str> = metadata["workspace_members"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect();
    metadata["packages"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(move |p| ids.contains(&p["id"].as_str().unwrap_or("")))
}

fn clean_repo(url: &str) -> Option<String> {
    let url = url.trim().trim_end_matches('/').trim_end_matches(".git");
    url.starts_with("https://").then(|| url.to_string())
}

/// `name = "1"` or `name = { version = "1", features = ["full"] }`, as written in `Cargo.toml`.
fn snippet(name: &str, dep: &Value) -> String {
    let req = str_of(dep, "req");
    let req = req.strip_prefix('^').unwrap_or(&req);
    let features: Vec<String> = dep["features"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(|f| format!("\"{f}\""))
        .collect();
    let default_features = dep["uses_default_features"].as_bool().unwrap_or(true);
    if features.is_empty() && default_features {
        return format!("{name} = \"{req}\"");
    }
    let mut fields = vec![format!("version = \"{req}\"")];
    if !default_features {
        fields.push("default-features = false".to_string());
    }
    if !features.is_empty() {
        fields.push(format!("features = [{}]", features.join(", ")));
    }
    format!("{name} = {{ {} }}", fields.join(", "))
}

fn cargo_metadata(manifest: &Path) -> Result<Value, String> {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--manifest-path"])
        .arg(manifest)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}

/// Size and modification time of every manifest and lockfile; changes whenever one is edited.
fn fingerprint(root: &Path) -> String {
    let mut files = Vec::new();
    find_manifests(root, &mut files);
    files.sort();
    let mut key = String::new();
    for path in files {
        if let Ok(m) = fs::metadata(&path) {
            let nanos = m
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map_or(0, |d| d.as_nanos());
            key.push_str(&format!("{}:{}:{nanos}\n", path.display(), m.len()));
        }
    }
    key
}

fn read_cache(path: &Path, key: &str) -> Option<Meta> {
    let cached: Value = serde_json::from_str(&fs::read_to_string(path).ok()?).ok()?;
    if cached["key"].as_str()? != key {
        return None;
    }
    let crates = cached["crates"].as_object()?;
    Some(Meta(
        crates
            .iter()
            .map(|(name, v)| {
                let field = |i: usize| v[i].as_str().map(str::to_string);
                (
                    name.clone(),
                    CrateMeta {
                        snippet: field(0),
                        repo: field(1),
                    },
                )
            })
            .collect(),
    ))
}

fn write_cache(path: &Path, key: &str, meta: &Meta) {
    let crates: serde_json::Map<String, Value> = meta
        .0
        .iter()
        .map(|(name, m)| (name.clone(), serde_json::json!([m.snippet, m.repo])))
        .collect();
    let body = serde_json::json!({ "key": key, "crates": crates });
    // A missing cache only costs time, so ignore write failures.
    let _ = fs::write(path, body.to_string());
}

fn find_manifests(dir: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let skip = path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| matches!(n, "target" | "node_modules" | "book" | ".git"));
            if !skip {
                find_manifests(&path, found);
            }
        } else if path
            .file_name()
            .is_some_and(|n| n == "Cargo.toml" || n == "Cargo.lock")
        {
            found.push(path);
        }
    }
}

// ---- rendering ----

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn crate_chip(name: &str, href: &str, meta: &CrateMeta) -> String {
    let title = if name == "std" {
        "Open the std documentation".to_string()
    } else {
        format!("Open {name} on docs.rs")
    };
    let mut html = format!(
        "<span class=\"crate-chip\"><a class=\"crate-chip-name\" href=\"{}\" title=\"{}\">{}</a>",
        escape(href),
        escape(&title),
        escape(name)
    );
    // std ships with Rust, so there is nothing to add to Cargo.toml.
    if name != "std" {
        let line = meta.snippet.as_deref().unwrap_or(name);
        html.push_str(&format!(
            "<button type=\"button\" class=\"crate-chip-copy\" data-copy=\"{0}\" title=\"Copy {0}\" aria-label=\"Copy {0}\">{COPY_ICON}</button>",
            escape(line)
        ));
    }
    if let Some(repo) = &meta.repo {
        let name = escape(name);
        html.push_str(&format!(
            "<a class=\"crate-chip-repo\" href=\"{}\" title=\"Open {name} source repository\" aria-label=\"{name} source repository\">{GITHUB_ICON}</a>",
            escape(repo)
        ));
    }
    html.push_str("</span>");
    html
}

fn category_label(slug: &str) -> String {
    match slug.to_lowercase().as_str() {
        "os" => return "OS".into(),
        "wasm" => return "WebAssembly".into(),
        "no_std" => return "no_std".into(),
        _ => {}
    }
    let text = slug.replace('_', " ");
    let mut chars = text.chars();
    chars
        .next()
        .map(|c| c.to_uppercase().chain(chars).collect())
        .unwrap_or_default()
}

/// The pill for a `[![x-badge]][x]` reference, or `None` if it is not one we know.
fn chip_for(badge_url: &str, href: &str, meta: &Meta) -> Option<String> {
    if let Some((_, rest)) = badge_url.split_once("img.shields.io/crates/v/") {
        let name = rest.split(".svg").next()?;
        return Some(crate_chip(name, href, &meta.get(name)));
    }
    let rest = badge_url.split_once("img.shields.io/badge/")?.1;
    if rest.starts_with("std-") {
        return Some(crate_chip("std", href, &meta.get("std")));
    }
    let slug = rest.split_once("--x")?.0;
    Some(format!(
        "<a class=\"crate-chip cat-chip\" href=\"{}\">{}</a>",
        escape(href),
        escape(&category_label(slug))
    ))
}

pub fn render_chapter(content: &str, meta: &Meta) -> String {
    let mut links = HashMap::new();
    for line in content.lines() {
        if let Some((label, url)) = search_index::parse_link_definition(line) {
            links.insert(label, url);
        }
    }

    let mut out = String::with_capacity(content.len());
    let mut rest = content;
    while let Some(start) = rest.find("[![") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 3..];
        let parsed = after.split_once("]][").and_then(|(badge, tail)| {
            let (label, tail) = tail.split_once(']')?;
            let bad = |s: &str| s.contains(['[', ']', '\n']);
            if bad(badge) || bad(label) || !badge.ends_with("-badge") {
                return None;
            }
            let chip = chip_for(links.get(badge)?, links.get(label)?, meta)?;
            Some((chip, tail))
        });
        match parsed {
            Some((chip, tail)) => {
                out.push_str(&chip);
                rest = tail;
            }
            None => {
                out.push_str("[![");
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

fn walk(value: &mut Value, meta: &Meta, version: &str) {
    match value {
        Value::Object(map) => {
            if let Some(Value::String(content)) = map.get_mut("content") {
                *content = render_chapter(content, meta).replace("{{mdbook-version}}", version);
            }
            for child in map.values_mut() {
                walk(child, meta, version);
            }
        }
        Value::Array(items) => items.iter_mut().for_each(|v| walk(v, meta, version)),
        _ => {}
    }
}

/// `mdbook-chips [supports <renderer>]`, the mdBook preprocessor protocol.
pub fn run_preprocess(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.first().map(String::as_str) == Some("supports") {
        let html = args.get(1).map(String::as_str) == Some("html");
        std::process::exit(if html { 0 } else { 1 });
    }

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let mut data: Value = serde_json::from_str(&input)?;
    let mut book = data
        .as_array_mut()
        .and_then(|a| a.pop())
        .ok_or("expected [context, book] on stdin")?;

    // The version of mdBook that is running this build, for `{{mdbook-version}}` in the text.
    let version = data[0]["mdbook_version"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();

    let meta = Meta::load()?;
    search_index::write_index(&meta)?;
    walk(&mut book, &meta, &version);

    let mut stdout = std::io::stdout();
    serde_json::to_writer(&mut stdout, &book)?;
    stdout.flush()?;
    Ok(())
}
