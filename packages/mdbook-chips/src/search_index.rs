//! Generates `theme/search-index.js`: crates, topics and recipes extracted from the section
//! tables in `src/*.md`. The browser scans it directly, so there is nothing to precompute.

use crate::chips::Meta;
use crate::project_root;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::{error::Error, fs};

const SKIPPED_PAGES: &[&str] = &["SUMMARY", "about", "intro", "links"];

struct Section {
    title: String,
    url: String,
    aliases: BTreeSet<String>,
}

/// A bold label above a table (`**Directory Traversal**`) that groups the recipes under it.
struct Group {
    section: usize,
    label: String,
    anchor: String,
}

struct Recipe {
    title: String,
    url: String,
    section: usize,
    group: Option<usize>,
    crates: Vec<usize>,
}

/// `**Label**` on a line of its own.
pub fn group_label(line: &str) -> Option<&str> {
    let label = line.trim().strip_prefix("**")?.strip_suffix("**")?;
    (!label.is_empty() && !label.contains("**")).then_some(label)
}

/// The id of a group label on its page, used for links from the search results.
pub fn group_anchor(label: &str) -> String {
    let mut slug = String::from("group");
    let mut dash = true;
    for c in label.to_lowercase().chars() {
        if c.is_alphanumeric() {
            if dash {
                slug.push('-');
            }
            slug.push(c);
            dash = false;
        } else {
            dash = true;
        }
    }
    slug
}

/// Writes `src/cookbook-index.js` (untracked; mdBook copies it into the book). It is only
/// rewritten when the content changes, so `mdbook serve` does not rebuild in a loop.
pub fn write_index(meta: &Meta) -> Result<(), Box<dyn Error>> {
    let output = generate(meta)?;
    let path = project_root().join("src").join("cookbook-index.js");
    if fs::read_to_string(&path).ok().as_deref() != Some(output.as_str()) {
        fs::write(&path, output)?;
    }
    Ok(())
}

fn generate(meta: &Meta) -> Result<String, Box<dyn Error>> {
    let src = project_root().join("src");
    let mut pages = Vec::new();
    let mut link_sources = Vec::new();
    for entry in fs::read_dir(&src)? {
        let path = entry?.path();
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        if path.extension().is_some_and(|e| e == "md") {
            let text = fs::read_to_string(&path)?;
            if !SKIPPED_PAGES.contains(&stem) {
                pages.push((stem.to_string(), text.clone()));
            }
            link_sources.push(text);
        }
    }
    // Sections follow the sidebar (SUMMARY.md); sorting by file name would put `wasm` before `web`.
    let sidebar = fs::read_to_string(src.join("SUMMARY.md"))?;
    let order: Vec<&str> = sidebar
        .lines()
        .filter_map(|l| l.strip_prefix("- ["))
        .filter_map(|l| l.split_once("](")?.1.strip_suffix(".md)"))
        .collect();
    pages.sort_by_key(|(stem, _)| {
        (
            order.iter().position(|o| o == stem).unwrap_or(usize::MAX),
            stem.clone(),
        )
    });

    let mut links = HashMap::new();
    for text in &link_sources {
        for line in text.lines() {
            if let Some((label, url)) = parse_link_definition(line) {
                links.insert(label, url);
            }
        }
    }

    let mut sections = Vec::new();
    let mut crates: BTreeMap<String, usize> = BTreeMap::new();
    let mut recipes = Vec::new();
    let mut groups: Vec<Group> = Vec::new();

    for (stem, text) in &pages {
        let Some(title) = text.lines().next().and_then(|l| l.strip_prefix("# ")) else {
            continue;
        };
        let section = sections.len();
        let mut aliases = BTreeSet::new();
        let first_recipe = recipes.len();

        let mut pending: Option<String> = None;
        let mut group = None;
        for line in text.lines() {
            if let Some(label) = group_label(line) {
                pending = Some(label.to_string());
            } else if line.starts_with("| Recipe") {
                group = pending.take().map(|label| {
                    groups.push(Group {
                        section,
                        anchor: group_anchor(&label),
                        label,
                    });
                    groups.len() - 1
                });
            }
            if !line.starts_with("| [") {
                continue;
            }
            let cols: Vec<&str> = line.trim_matches('|').split('|').collect();
            let Some((recipe_title, label)) = parse_link(cols[0].trim()) else {
                continue;
            };
            let Some(url) = links.get(&label) else {
                continue;
            };
            let mut recipe_crates = Vec::new();
            for badge in cols.iter().skip(1).flat_map(|c| badges(c)) {
                if let Some(category) = badge.strip_prefix("cat-") {
                    aliases.insert(category.replace('-', " "));
                } else {
                    let name = crate_name(&links, &badge);
                    let next = crates.len();
                    recipe_crates.push(*crates.entry(name).or_insert(next));
                }
            }
            recipes.push(Recipe {
                title: recipe_title,
                url: url.clone(),
                section,
                group,
                crates: recipe_crates,
            });
        }
        if recipes.len() > first_recipe {
            sections.push(Section {
                title: title.to_string(),
                url: format!("{stem}.html"),
                aliases,
            });
        }
    }

    // Crates are numbered in discovery order; renumber alphabetically for stable output.
    let names: Vec<String> = crates.keys().cloned().collect();
    let remap: HashMap<usize, usize> = names
        .iter()
        .enumerate()
        .map(|(new, name)| (crates[name], new))
        .collect();
    for recipe in &mut recipes {
        recipe.crates = recipe.crates.iter().map(|c| remap[c]).collect();
        recipe.crates.sort();
        recipe.crates.dedup();
    }

    let mut out =
        String::from("// Generated by `mdbook-chips` during the book build. Do not edit.\n");
    out.push_str("window.COOKBOOK_INDEX = {\n");
    out.push_str(&format!("  \"crates\": {},\n", json_strings(names.iter())));
    let rows: Vec<String> = names
        .iter()
        .map(|n| {
            let m = meta.get(n);
            let field = |v: Option<String>| v.map_or("null".to_string(), |s| json_str(&s));
            let playground = m.playground.then(|| m.playground_href().to_string());
            format!(
                "[{}, {}, {}]",
                field(m.snippet),
                field(m.repo),
                field(playground)
            )
        })
        .collect();
    out.push_str(&format!("  \"meta\": [{}],\n", rows.join(", ")));
    out.push_str("  \"sections\": [\n");
    let rows: Vec<String> = sections
        .iter()
        .map(|s| {
            format!(
                "    [{}, {}, {}]",
                json_str(&s.title),
                json_str(&s.url),
                json_strings(s.aliases.iter())
            )
        })
        .collect();
    out.push_str(&rows.join(",\n"));
    out.push_str("\n  ],\n  \"groups\": [\n");
    let rows: Vec<String> = groups
        .iter()
        .enumerate()
        .map(|(i, g)| {
            let count = recipes.iter().filter(|r| r.group == Some(i)).count();
            format!(
                "    [{}, {}, {}, {}]",
                g.section,
                json_str(&g.label),
                json_str(&g.anchor),
                count
            )
        })
        .collect();
    out.push_str(&rows.join(",\n"));
    out.push_str("\n  ],\n  \"recipes\": [\n");
    let rows: Vec<String> = recipes
        .iter()
        .map(|r| {
            let ids: Vec<String> = r.crates.iter().map(|c| c.to_string()).collect();
            format!(
                "    [{}, {}, {}, [{}], {}]",
                json_str(&r.title),
                json_str(&r.url),
                r.section,
                ids.join(","),
                r.group.map_or(-1, |g| g as i64)
            )
        })
        .collect();
    out.push_str(&rows.join(",\n"));
    out.push_str("\n  ]\n};\n");
    Ok(out)
}

/// `[label]: url`
pub fn parse_link_definition(line: &str) -> Option<(String, String)> {
    let rest = line.strip_prefix('[')?;
    let (label, url) = rest.split_once("]:")?;
    let url = url.trim();
    (!url.is_empty() && !label.contains(['[', ']'])).then(|| (label.to_string(), url.to_string()))
}

/// `[text][label]`
pub fn parse_link(cell: &str) -> Option<(String, String)> {
    let rest = cell.strip_prefix('[')?;
    let (text, rest) = rest.split_once("][")?;
    let label = rest.strip_suffix(']')?;
    Some((text.to_string(), label.to_string()))
}

/// The real crate name behind a badge label (`serde-json` is `serde_json`), read from the
/// badge's `img.shields.io/crates/v/<name>.svg` URL.
fn crate_name(links: &HashMap<String, String>, label: &str) -> String {
    links
        .get(&format!("{label}-badge"))
        .and_then(|url| url.split_once("/crates/v/"))
        .and_then(|(_, rest)| rest.split_once(".svg"))
        .map(|(name, _)| name.to_string())
        .unwrap_or_else(|| label.to_string())
}

/// Names `x` from every `[![x-badge]]` in a table cell.
pub fn badges(cell: &str) -> Vec<String> {
    cell.split("[![")
        .skip(1)
        .filter_map(|part| part.split_once("-badge]").map(|(name, _)| name.to_string()))
        .collect()
}

fn json_strings<'a>(items: impl Iterator<Item = &'a String>) -> String {
    let items: Vec<String> = items.map(|s| json_str(s)).collect();
    format!("[{}]", items.join(", "))
}

fn json_str(s: &str) -> String {
    let mut out = String::from('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
