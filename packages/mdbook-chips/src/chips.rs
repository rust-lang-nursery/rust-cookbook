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
const PLAYGROUND_ICON: &str = r#"<svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path fill="currentColor" d="M4 2.2v11.6a.6.6 0 0 0 .92.5l9-5.8a.6.6 0 0 0 0-1l-9-5.8A.6.6 0 0 0 4 2.2z"/></svg>"#;
const PLAYGROUND_URL: &str = "https://play.rust-lang.org/";
/// Where the per-crate Playground starters live: `<crate name, `-` as `_`>.rs`, one per crate.
const STARTER_DIR: &str = "crates/playground/src/bin";
const STARTER_QUERY: &str = "?version=stable&mode=debug&edition=2024&code=";
const GITHUB_ICON: &str = r#"<svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path fill="currentColor" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82a7.6 7.6 0 0 1 4 0c1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8z"/></svg>"#;

/// What the pill for one crate offers: a `Cargo.toml` line to copy, a repository link and whether
/// the cookbook's root `[dependencies]` put the crate on the Rust Playground.
#[derive(Default, Clone)]
pub struct CrateMeta {
    /// The line to copy without any features: `name = "1.2"`.
    pub snippet: Option<String>,
    /// The crate's name in `Cargo.toml` and its version requirement, to write a line with features.
    pub dep: Option<String>,
    pub req: Option<String>,
    pub repo: Option<String>,
    pub playground: bool,
    /// The Playground link with the crate's starter code filled in; `None` if it has no starter.
    pub starter: Option<String>,
}

impl CrateMeta {
    /// The line to copy for a recipe whose banner enables `features`; the plain line when none.
    pub fn line(&self, features: &[String]) -> Option<String> {
        if features.is_empty() {
            return self.snippet.clone();
        }
        let (dep, req) = (self.dep.as_ref()?, self.req.as_ref()?);
        let list: Vec<String> = features.iter().map(|f| format!("\"{f}\"")).collect();
        Some(format!(
            "{dep} = {{ version = \"{req}\", features = [{}] }}",
            list.join(", ")
        ))
    }

    /// Where the play icon points: the crate's starter, or the blank Playground.
    pub fn playground_href(&self) -> &str {
        self.starter.as_deref().unwrap_or(PLAYGROUND_URL)
    }
}

pub struct Meta(HashMap<String, CrateMeta>);

#[cfg(test)]
impl Meta {
    /// A `Meta` that knows these crates at version 1, so a test can render pages without running
    /// `cargo metadata`.
    pub fn for_crates<'a>(names: impl IntoIterator<Item = &'a str>) -> Meta {
        Meta(
            names
                .into_iter()
                .map(|name| {
                    let meta = CrateMeta {
                        snippet: Some(format!("{name} = \"1\"")),
                        dep: Some(name.to_string()),
                        req: Some("1".to_string()),
                        ..CrateMeta::default()
                    };
                    (normalize(name), meta)
                })
                .collect(),
        )
    }
}

fn normalize(name: &str) -> String {
    name.to_lowercase().replace('-', "_")
}

impl Meta {
    pub fn get(&self, name: &str) -> CrateMeta {
        if name == "std" {
            return CrateMeta {
                repo: Some(STD_REPO.to_string()),
                ..CrateMeta::default()
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
        if let Some(mut meta) = read_cache(&cache, &key) {
            meta.attach_starters(&root);
            return Ok(meta);
        }
        let (mut meta, complete) = Meta::resolve(&root)?;
        if complete {
            write_cache(&cache, &key, &meta);
        }
        meta.attach_starters(&root);
        Ok(meta)
    }

    /// Points each Playground crate's icon at its starter. The starters are read on every build
    /// instead of being cached, so editing one is picked up without touching a `Cargo.toml`.
    fn attach_starters(&mut self, root: &Path) {
        for (name, meta) in self.0.iter_mut().filter(|(_, m)| m.playground) {
            match fs::read_to_string(root.join(STARTER_DIR).join(format!("{name}.rs"))) {
                Ok(code) => meta.starter = Some(starter_url(&code)),
                Err(_) => eprintln!(
                    "warning: no Playground starter for `{name}` ({STARTER_DIR}/{name}.rs)"
                ),
            }
        }
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

        // The playground reads the keys of the root package's `[dependencies]`, so those are the
        // crates the cookbook adds to it (optional ones included; dev and build dependencies are not).
        let playground: Vec<String> = members(&main)
            .filter(|p| str_of(p, "name") == "rust-cookbook")
            .flat_map(|p| p["dependencies"].as_array().into_iter().flatten())
            .filter(|dep| dep["kind"].is_null())
            .map(|dep| normalize(&str_of(dep, "name")))
            .collect();

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
                    let req = str_of(dep, "req");
                    let req = req.strip_prefix('^').unwrap_or(&req).to_string();
                    out.entry(normalize(&name)).or_insert_with(|| CrateMeta {
                        snippet: Some(format!("{name} = \"{req}\"")),
                        dep: Some(name),
                        req: Some(req),
                        ..CrateMeta::default()
                    });
                }
            }
        }
        for (name, meta) in &mut out {
            meta.repo = repos.get(name).cloned();
            meta.playground = playground.contains(name);
        }
        Ok((Meta(out), complete))
    }
}

/// A Playground link that opens with `code` loaded (`?code=` is read by the Playground's frontend).
fn starter_url(code: &str) -> String {
    let mut url = format!("{PLAYGROUND_URL}{STARTER_QUERY}");
    for byte in code.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                url.push(byte as char)
            }
            _ => url.push_str(&format!("%{byte:02X}")),
        }
    }
    url
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
    // Bumped when the cached fields change, so an old cache is not read as the new shape.
    let mut key = String::from("v3\n");
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
                        playground: v[2].as_bool().unwrap_or(false),
                        dep: field(3),
                        req: field(4),
                        starter: None,
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
        .map(|(name, m)| {
            (
                name.clone(),
                serde_json::json!([m.snippet, m.repo, m.playground, m.dep, m.req]),
            )
        })
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

fn crate_chip(name: &str, href: &str, meta: &CrateMeta, features: &[String]) -> String {
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
        let line = meta.line(features).unwrap_or_else(|| name.to_string());
        html.push_str(&format!(
            "<button type=\"button\" class=\"crate-chip-copy\" data-copy=\"{0}\" title=\"Copy {0}\" aria-label=\"Copy {0}\">{COPY_ICON}</button>",
            escape(&line)
        ));
    }
    if meta.playground {
        html.push_str(&format!(
            "<a class=\"crate-chip-playground\" href=\"{1}\" title=\"Try {0} in the Rust Playground\" aria-label=\"Try {0} in the Rust Playground\">{PLAYGROUND_ICON}</a>",
            escape(name),
            escape(meta.playground_href())
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
fn chip_for(
    badge_url: &str,
    href: &str,
    meta: &Meta,
    features: &dyn Fn(&str) -> Vec<String>,
) -> Option<String> {
    if let Some((_, rest)) = badge_url.split_once("img.shields.io/crates/v/") {
        let name = rest.split(".svg").next()?;
        return Some(crate_chip(name, href, &meta.get(name), &features(name)));
    }
    let rest = badge_url.split_once("img.shields.io/badge/")?.1;
    if rest.starts_with("std-") {
        return Some(crate_chip("std", href, &meta.get("std"), &[]));
    }
    let slug = rest.split_once("--x")?.0;
    Some(format!(
        "<a class=\"crate-chip cat-chip\" href=\"{}\">{}</a>",
        escape(href),
        escape(&category_label(slug))
    ))
}

/// A bold group label directly above a recipe table becomes a paragraph with an id, so the search
/// results can link to the group.
fn anchor_group_labels(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut out = String::with_capacity(content.len());
    for (i, line) in lines.iter().enumerate() {
        let before_table = lines[i + 1..]
            .iter()
            .find(|l| !l.trim().is_empty())
            .is_some_and(|l| l.starts_with("| Recipe"));
        match search_index::group_label(line) {
            Some(label) if before_table => out.push_str(&format!(
                "<p id=\"{}\"><strong>{}</strong></p>",
                search_index::group_anchor(label),
                escape(label)
            )),
            _ => out.push_str(line),
        }
        out.push('\n');
    }
    out
}

/// The features a recipe's banner names, by crate, in the order written. A banner reads
/// "> This recipe requires the [`macros`] and [`time`] feature flags of `tokio` to be enabled in
/// `Cargo.toml`." and may name more crates ("... and the [`full`] feature flag of `tokio`").
pub fn banner_features(markdown: &str) -> HashMap<String, Vec<String>> {
    let mut out: HashMap<String, Vec<String>> = HashMap::new();
    for line in markdown
        .lines()
        .filter(|l| l.starts_with("> This recipe requires the "))
    {
        let mut rest = line;
        while let Some(at) = rest.find(" feature flag") {
            let (before, after) = rest.split_at(at);
            let Some(of) = after.find(" of `") else { break };
            let tail = &after[of + " of `".len()..];
            let Some(end) = tail.find('`') else { break };
            let named = out.entry(tail[..end].to_string()).or_default();
            for part in before.split("[`").skip(1) {
                if let Some((feature, _)) = part.split_once("`]") {
                    if !named.iter().any(|f| f == feature) {
                        named.push(feature.to_string());
                    }
                }
            }
            rest = &tail[end + 1..];
        }
    }
    out
}

/// A heading of a chapter and the banners in the text below it, up to the next heading.
struct Section {
    start: usize,
    level: usize,
    banners: HashMap<String, Vec<String>>,
}

fn sections(content: &str) -> Vec<Section> {
    let mut out = vec![Section {
        start: 0,
        level: 0,
        banners: HashMap::new(),
    }];
    let (mut offset, mut fenced) = (0, false);
    let mut texts = vec![String::new()];
    for line in content.split_inclusive('\n') {
        if line.starts_with("```") {
            fenced = !fenced;
        }
        let level = line.bytes().take_while(|&b| b == b'#').count();
        if !fenced && (1..=6).contains(&level) && line[level..].starts_with(' ') {
            out.push(Section {
                start: offset,
                level,
                banners: HashMap::new(),
            });
            texts.push(String::new());
        }
        texts.last_mut().unwrap().push_str(line);
        offset += line.len();
    }
    for (section, text) in out.iter_mut().zip(texts) {
        section.banners = banner_features(&text);
    }
    out
}

/// The features the banner for `krate` names at `offset`: the one in the section the chip is in, or
/// failing that in the nearest section above it (the shared banner at the top of a page).
fn features_at(sections: &[Section], offset: usize, krate: &str) -> Vec<String> {
    let mut at = sections.partition_point(|s| s.start <= offset);
    let mut level = usize::MAX;
    while at > 0 {
        at -= 1;
        if sections[at].level < level {
            level = sections[at].level;
            let named = sections[at]
                .banners
                .iter()
                .find(|(name, _)| normalize(name) == normalize(krate));
            if let Some((_, features)) = named {
                return features.clone();
            }
        }
    }
    Vec::new()
}

pub fn render_chapter(content: &str, meta: &Meta) -> String {
    let anchored = anchor_group_labels(content);
    let content = anchored.as_str();
    let mut links = HashMap::new();
    for line in content.lines() {
        if let Some((label, url)) = search_index::parse_link_definition(line) {
            links.insert(label, url);
        }
    }

    let sections = sections(content);
    let mut out = String::with_capacity(content.len());
    let mut rest = content;
    while let Some(start) = rest.find("[![") {
        out.push_str(&rest[..start]);
        let offset = content.len() - rest.len() + start;
        let features = |krate: &str| features_at(&sections, offset, krate);
        let after = &rest[start + 3..];
        let parsed = after.split_once("]][").and_then(|(badge, tail)| {
            let (label, tail) = tail.split_once(']')?;
            let bad = |s: &str| s.contains(['[', ']', '\n']);
            if bad(badge) || bad(label) || !badge.ends_with("-badge") {
                return None;
            }
            let chip = chip_for(links.get(badge)?, links.get(label)?, meta, &features)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playground_icon_only_for_crates_in_the_root_dependencies() {
        let on = CrateMeta {
            playground: true,
            ..CrateMeta::default()
        };
        assert!(crate_chip("nalgebra", "https://docs.rs/nalgebra", &on, &[])
            .contains("crate-chip-playground"));
        let off = CrateMeta::default();
        assert!(
            !crate_chip("nalgebra", "https://docs.rs/nalgebra", &off, &[])
                .contains("crate-chip-playground")
        );
    }

    /// Every crate the root manifest lists for the Playground has a starter next to the others.
    #[test]
    fn every_root_dependency_has_a_starter() {
        let root = project_root();
        let manifest = fs::read_to_string(root.join("Cargo.toml")).expect("root Cargo.toml");
        let names = manifest
            .lines()
            .skip_while(|l| l.trim() != "[dependencies]")
            .skip(1)
            .take_while(|l| !l.starts_with('['))
            .filter_map(|l| {
                l.split_once(" = ")
                    .map(|(name, _)| name.trim().replace('-', "_"))
            });
        let missing: Vec<String> = names
            .filter(|name| !root.join(STARTER_DIR).join(format!("{name}.rs")).exists())
            .collect();
        assert!(
            missing.is_empty(),
            "no starter in {STARTER_DIR} for {missing:?}"
        );
    }

    fn tokio_meta() -> Meta {
        let tokio = CrateMeta {
            snippet: Some("tokio = \"1\"".to_string()),
            dep: Some("tokio".to_string()),
            req: Some("1".to_string()),
            ..CrateMeta::default()
        };
        Meta(HashMap::from([("tokio".to_string(), tokio)]))
    }

    const LINKS: &str = "\n[tokio-badge]: https://img.shields.io/crates/v/tokio.svg\n[tokio]: https://docs.rs/tokio\n";

    fn copied(html: &str) -> Vec<&str> {
        html.split("data-copy=\"")
            .skip(1)
            .filter_map(|part| part.split('"').next())
            .collect()
    }

    #[test]
    fn copy_line_has_the_features_of_the_recipes_banner() {
        let page = format!(
            "## One\n[![tokio-badge]][tokio]\n\n> This recipe requires the [`macros`] and [`time`] feature flags of `tokio` to be enabled in `Cargo.toml`.\n\n## Two\n[![tokio-badge]][tokio]\n{LINKS}"
        );
        assert_eq!(
            copied(&render_chapter(&page, &tokio_meta())),
            [
                "tokio = { version = &quot;1&quot;, features = [&quot;macros&quot;, &quot;time&quot;] }",
                // The banner belongs to the recipe above, not to the next one.
                "tokio = &quot;1&quot;",
            ]
        );
    }

    #[test]
    fn a_banner_at_the_top_of_a_page_covers_the_recipes_in_it() {
        let page = format!(
            "# Page\n\n> This recipe requires the [`full`] feature flag of `tokio` to be enabled in `Cargo.toml`.\n\n## One\n[![tokio-badge]][tokio]\n{LINKS}"
        );
        assert_eq!(
            copied(&render_chapter(&page, &tokio_meta())),
            ["tokio = { version = &quot;1&quot;, features = [&quot;full&quot;] }"]
        );
    }

    #[test]
    fn a_table_chip_copies_the_plain_line() {
        let page = format!("| Recipe | Crates |\n| [A][a] | [![tokio-badge]][tokio] |\n{LINKS}");
        assert_eq!(
            copied(&render_chapter(&page, &tokio_meta())),
            ["tokio = &quot;1&quot;"]
        );
    }
}
