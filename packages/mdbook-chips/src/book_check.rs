//! Tests on the book's own structure: every recipe is listed in its section's table, in the order
//! of the sidebar and of the recipe pages. They read `src/` the way mdBook does (expanding
//! `{{#include}}`, numbering headings the same way), so they need no build.

use crate::chips::{banner_features, render_chapter, Meta};
use crate::project_root;
use crate::search_index::{badges, parse_link, parse_link_definition};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::Path;

/// Put this on the line below a heading that is a guide or an explanation rather than a recipe, so
/// the recipe tests skip it.
const NOT_A_RECIPE: &str = "<!-- not-a-recipe -->";

struct Entry {
    depth: usize,
    path: String,
}

struct Heading {
    level: usize,
    text: String,
    id: String,
    /// Byte offset in the markdown just after the heading.
    end: usize,
}

struct Row {
    title: String,
    page: String,
    fragment: Option<String>,
    /// The badge labels (`clap`, `cat-command-line`) in the row's Crates and Categories cells.
    badges: BTreeSet<String>,
}

struct Book {
    sidebar: Vec<Entry>,
    links: HashMap<String, String>,
}

impl Book {
    fn load() -> Book {
        let src = project_root().join("src");
        let summary = fs::read_to_string(src.join("SUMMARY.md")).unwrap();
        let sidebar = summary
            .lines()
            .filter_map(|line| {
                let indent = line.len() - line.trim_start().len();
                let rest = line.trim_start().strip_prefix("- [")?;
                let path = rest.split_once("](")?.1.strip_suffix(')')?;
                Some(Entry {
                    depth: indent / 2,
                    path: path.to_string(),
                })
            })
            .collect();
        let mut links = HashMap::new();
        for entry in fs::read_dir(&src).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|e| e == "md") {
                for line in fs::read_to_string(&path).unwrap().lines() {
                    if let Some((label, url)) = parse_link_definition(line) {
                        links.insert(label, url);
                    }
                }
            }
        }
        Book { sidebar, links }
    }

    /// Top-level pages of the sidebar.
    fn sections(&self) -> impl Iterator<Item = (usize, &Entry)> {
        self.sidebar
            .iter()
            .enumerate()
            .filter(|(_, e)| e.depth == 0)
    }

    /// The pages below a section in the sidebar, in order, as indexes into `sidebar`.
    fn descendants(&self, section: usize) -> Vec<usize> {
        (section + 1..self.sidebar.len())
            .take_while(|&i| self.sidebar[i].depth > 0)
            .collect()
    }

    fn is_leaf(&self, i: usize) -> bool {
        self.sidebar
            .get(i + 1)
            .is_none_or(|next| next.depth <= self.sidebar[i].depth)
    }

    fn position(&self, page: &str) -> Option<usize> {
        self.sidebar.iter().position(|e| e.path == page)
    }

    /// The recipe rows of a section page, in file order, with the page and heading each links to.
    fn rows(&self, section: &str) -> Vec<Row> {
        let text = fs::read_to_string(project_root().join("src").join(section)).unwrap();
        let mut rows = Vec::new();
        for line in text.lines().filter(|l| l.starts_with("| [")) {
            let mut cells = line.trim_matches('|').split('|');
            let first = cells.next().unwrap_or("");
            let Some((title, label)) = parse_link(first.trim()) else {
                continue;
            };
            let Some(url) = self.links.get(&label) else {
                panic!("{section}: `{title}` uses the undefined link [{label}]");
            };
            let (page, fragment) = match url.split_once('#') {
                Some((page, fragment)) => (page, Some(fragment.to_string())),
                None => (url.as_str(), None),
            };
            rows.push(Row {
                title,
                page: page.replace(".html", ".md"),
                fragment,
                badges: cells.flat_map(badges).collect(),
            });
        }
        rows
    }
}

/// A markdown file with its `{{#include file.md}}` lines replaced by the included text.
fn expand(path: &Path) -> String {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let dir = path.parent().unwrap();
    let mut out = String::new();
    for line in text.lines() {
        let include = line
            .trim()
            .strip_prefix("{{#include ")
            .and_then(|l| l.strip_suffix("}}"))
            .map(str::trim);
        match include {
            Some(file) if file.ends_with(".md") => out.push_str(&expand(&dir.join(file))),
            Some(_) => {}
            None => out.push_str(line),
        }
        out.push('\n');
    }
    out
}

/// mdBook's heading ids: lowercase letters and digits kept, whitespace to `-`, the rest dropped,
/// and a counter for repeats.
fn headings(markdown: &str) -> Vec<Heading> {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_HEADING_ATTRIBUTES
        | Options::ENABLE_SMART_PUNCTUATION;
    let mut out: Vec<Heading> = Vec::new();
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut open: Option<(usize, String)> = None;
    for (event, range) in Parser::new_ext(markdown, options).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                open = Some((level_number(level), String::new()))
            }
            Event::Text(t) | Event::Code(t) => {
                if let Some((_, text)) = open.as_mut() {
                    text.push_str(&t);
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((level, text)) = open.take() {
                    let base: String = text
                        .chars()
                        .filter_map(|c| {
                            if c.is_alphanumeric() || c == '_' || c == '-' {
                                Some(c.to_ascii_lowercase())
                            } else if c.is_whitespace() {
                                Some('-')
                            } else {
                                None
                            }
                        })
                        .collect();
                    let n = counts.entry(base.clone()).or_insert(0);
                    let id = if *n == 0 { base } else { format!("{base}-{n}") };
                    *n += 1;
                    out.push(Heading {
                        level,
                        text,
                        id,
                        end: range.end,
                    });
                }
            }
            _ => {}
        }
    }
    out
}

fn level_number(level: HeadingLevel) -> usize {
    level as usize
}

/// Whether the line below `heading` is the `NOT_A_RECIPE` marker.
fn is_marked_not_a_recipe(markdown: &str, heading: &Heading) -> bool {
    markdown[heading.end..]
        .lines()
        .find(|l| !l.trim().is_empty())
        .is_some_and(|l| l.trim() == NOT_A_RECIPE)
}

fn page_headings(page: &str) -> Vec<Heading> {
    headings(&expand(&project_root().join("src").join(page)))
}

/// Section pages: the top-level pages whose table lists recipes.
fn recipe_sections(book: &Book) -> Vec<(usize, String)> {
    book.sections()
        .filter(|(_, e)| {
            fs::read_to_string(project_root().join("src").join(&e.path))
                .is_ok_and(|t| t.lines().any(|l| l.starts_with("| Recipe")))
        })
        .map(|(i, e)| (i, e.path.clone()))
        .collect()
}

#[test]
fn every_row_links_to_a_page_and_heading_that_exist() {
    let book = Book::load();
    let mut problems = Vec::new();
    for (_, section) in recipe_sections(&book) {
        for row in book.rows(&section) {
            if book.position(&row.page).is_none() {
                problems.push(format!(
                    "{section}: `{}` links to {}, which is not in the sidebar",
                    row.title, row.page
                ));
                continue;
            }
            if let Some(fragment) = &row.fragment {
                if !page_headings(&row.page).iter().any(|h| &h.id == fragment) {
                    problems.push(format!(
                        "{section}: `{}` links to #{fragment}, which is not a heading on {}",
                        row.title, row.page
                    ));
                }
            }
        }
    }
    assert!(problems.is_empty(), "\n{}\n", problems.join("\n"));
}

#[test]
fn every_recipe_is_listed_in_its_sections_table() {
    let book = Book::load();
    let mut problems = Vec::new();
    for (section_index, section) in recipe_sections(&book) {
        let rows = book.rows(&section);
        let whole_pages: HashSet<&str> = rows
            .iter()
            .filter(|r| r.fragment.is_none())
            .map(|r| r.page.as_str())
            .collect();
        let listed: HashSet<(&str, &str)> = rows
            .iter()
            .filter_map(|r| Some((r.page.as_str(), r.fragment.as_deref()?)))
            .collect();
        for i in book.descendants(section_index) {
            if !book.is_leaf(i) {
                continue;
            }
            let page = book.sidebar[i].path.as_str();
            if !rows.iter().any(|r| r.page == page) {
                problems.push(format!("{section}: no row links to {page}"));
                continue;
            }
            if whole_pages.contains(page) {
                continue;
            }
            // A heading counts as listed when it, or a heading above it (such as the page title of
            // a single-recipe page), has a row.
            // A heading that only groups recipes (`## C` over the `### ...` recipes below it) needs
            // no row of its own, and neither does one marked as not a recipe.
            let mut above: Vec<(usize, bool)> = Vec::new();
            let markdown = expand(&project_root().join("src").join(page));
            let all = headings(&markdown);
            for (n, h) in all.iter().enumerate() {
                above.retain(|(level, _)| *level < h.level);
                let own = listed.contains(&(page, h.id.as_str()));
                let covered = own || above.iter().any(|(_, listed)| *listed);
                above.push((h.level, own));
                let groups_recipes = all[n + 1..]
                    .iter()
                    .take_while(|next| next.level > h.level)
                    .any(|next| listed.contains(&(page, next.id.as_str())));
                if h.level >= 2
                    && !covered
                    && !groups_recipes
                    && !is_marked_not_a_recipe(&markdown, h)
                {
                    problems.push(format!(
                        "{section}: no row for \"{}\" (#{}) on {page}; add one, or put {NOT_A_RECIPE} under the heading if it is not a recipe",
                        h.text, h.id
                    ));
                }
            }
        }
    }
    assert!(problems.is_empty(), "\n{}\n", problems.join("\n"));
}

#[test]
fn table_rows_follow_the_sidebar_and_the_recipe_pages() {
    let book = Book::load();
    let mut problems = Vec::new();
    for (_, section) in recipe_sections(&book) {
        // (position of the page in the sidebar, position of the heading on the page)
        let mut previous: Option<((usize, usize), String)> = None;
        for row in book.rows(&section) {
            let Some(page) = book.position(&row.page) else {
                continue;
            };
            let heading = match &row.fragment {
                None => 0,
                Some(fragment) => page_headings(&row.page)
                    .iter()
                    .position(|h| &h.id == fragment)
                    .map_or(0, |p| p + 1),
            };
            let key = (page, heading);
            if let Some((before, title)) = &previous {
                if key < *before {
                    problems.push(format!(
                        "{section}: `{}` should come before `{title}`",
                        row.title
                    ));
                }
            }
            previous = Some((key, row.title));
        }
    }
    assert!(problems.is_empty(), "\n{}\n", problems.join("\n"));
}

#[test]
fn topics_are_in_the_same_order_in_the_sidebar_and_on_the_landing_page() {
    let book = Book::load();
    let sidebar: Vec<&str> = book.sections().map(|(_, e)| e.path.as_str()).collect();
    let intro = fs::read_to_string(project_root().join("src").join("intro.md")).unwrap();
    let grid: Vec<&str> = intro
        .lines()
        .filter_map(|l| l.strip_prefix("- ["))
        .filter_map(|l| l.split_once("](")?.1.strip_suffix(')'))
        .collect();
    let topics: Vec<&str> = sidebar
        .iter()
        .copied()
        .filter(|p| grid.contains(p))
        .collect();
    assert_eq!(
        topics, grid,
        "src/intro.md lists the topics in a different order than src/SUMMARY.md"
    );
}

/// The badge labels on the line under a heading (`[![clap-badge]][clap] [![cat-...]]`), or none if
/// the first thing under it is not a line of badges.
fn badges_under(markdown: &str, heading: &Heading) -> BTreeSet<String> {
    markdown[heading.end..]
        .lines()
        .find(|l| !l.trim().is_empty())
        .map(|l| badges(l).into_iter().collect())
        .unwrap_or_default()
}

#[test]
fn every_recipe_shows_the_same_chips_as_its_table_row() {
    let book = Book::load();
    let mut problems = Vec::new();
    for (_, section) in recipe_sections(&book) {
        for row in book.rows(&section) {
            if book.position(&row.page).is_none() {
                continue;
            }
            let markdown = expand(&project_root().join("src").join(&row.page));
            let all = headings(&markdown);
            let heading = match &row.fragment {
                Some(fragment) => all.iter().find(|h| &h.id == fragment),
                None => all.first(),
            };
            let Some(heading) = heading else { continue };
            let shown = badges_under(&markdown, heading);
            if shown == row.badges {
                continue;
            }
            let list = |set: &BTreeSet<String>| set.iter().cloned().collect::<Vec<_>>().join(", ");
            let missing: BTreeSet<String> = row.badges.difference(&shown).cloned().collect();
            let extra: BTreeSet<String> = shown.difference(&row.badges).cloned().collect();
            let mut why = Vec::new();
            if !missing.is_empty() {
                why.push(format!("missing {}", list(&missing)));
            }
            if !extra.is_empty() {
                why.push(format!("not in the table: {}", list(&extra)));
            }
            problems.push(format!(
                "{section}: `{}` ({}{}) {}",
                row.title,
                row.page,
                row.fragment
                    .as_ref()
                    .map_or(String::new(), |f| format!("#{f}")),
                why.join("; ")
            ));
        }
    }
    assert!(problems.is_empty(), "\n{}\n", problems.join("\n"));
}

/// The dependencies a recipe uses without needing the features its crate enables for other recipes,
/// from a line such as `<!-- needs-no-features: ndarray, nalgebra -->` anywhere in its markdown.
fn needs_no_features(markdown: &str) -> HashSet<String> {
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
fn mentions(text: &str, ident: &str) -> bool {
    let word = |c: char| c.is_alphanumeric() || c == '_';
    text.match_indices(ident).any(|(at, _)| {
        !text[..at].chars().next_back().is_some_and(word)
            && !text[at + ident.len()..].chars().next().is_some_and(word)
    })
}

/// The dependencies of a standalone crate that enable features, as `name -> features`.
fn dependencies_with_features(manifest: &Path) -> HashMap<String, BTreeSet<String>> {
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
fn included_crate_sources(page: &Path) -> Vec<(std::path::PathBuf, String)> {
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

fn markdown_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            markdown_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "md") {
            out.push(path);
        }
    }
}

/// A recipe whose code needs a Cargo feature says so in a banner (see CONTRIBUTING.md): when the
/// crate it includes enables features of a dependency the code uses, the recipe's file must have a
/// banner for that dependency, and the banner may only name features the crate really enables.
#[test]
fn recipes_needing_cargo_features_have_a_banner() {
    let src = project_root().join("src");
    let mut files = Vec::new();
    markdown_files(&src, &mut files);
    files.sort();
    let mut parents: HashMap<std::path::PathBuf, Vec<std::path::PathBuf>> = HashMap::new();
    for page in &files {
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
    let mut problems = Vec::new();
    for page in files {
        let sources = included_crate_sources(&page);
        if sources.is_empty() {
            continue;
        }
        let name = page.strip_prefix(&src).unwrap().display().to_string();
        // A banner on a page that includes this one (such as the shared banner at the top of the
        // SQLx page) counts for every recipe it includes.
        let mut banners: HashMap<String, Vec<String>> = HashMap::new();
        let mut pending = vec![page.canonicalize().unwrap()];
        let mut seen = HashSet::new();
        while let Some(file) = pending.pop() {
            if !seen.insert(file.clone()) {
                continue;
            }
            for (krate, features) in banner_features(&fs::read_to_string(&file).unwrap()) {
                banners.entry(krate).or_default().extend(features);
            }
            pending.extend(parents.get(&file).cloned().unwrap_or_default());
        }
        let text = fs::read_to_string(&page).unwrap();
        let exempt = needs_no_features(&text);
        let mut used = HashSet::new();
        for (dir, source) in &sources {
            for (dep, enabled) in dependencies_with_features(&dir.join("Cargo.toml")) {
                if !mentions(source, &dep.replace('-', "_")) {
                    continue;
                }
                used.insert(dep.clone());
                if exempt.contains(&dep) {
                    continue;
                }
                let Some(named) = banners.get(&dep) else {
                    problems.push(format!(
                        "{name}: needs a banner for the {} feature(s) of `{dep}` that {} enables, or `<!-- needs-no-features: {dep} -->` if it needs none",
                        enabled.iter().cloned().collect::<Vec<_>>().join(", "),
                        dir.file_name().unwrap().to_string_lossy()
                    ));
                    continue;
                };
                // `full` turns on every tokio feature, so a banner may name any of them.
                if !enabled.contains("full") {
                    for feature in named.iter().filter(|f| !enabled.contains(*f)) {
                        problems.push(format!(
                            "{name}: the banner names `{feature}` of `{dep}`, which the crate does not enable"
                        ));
                    }
                }
            }
        }
        for dep in exempt.difference(&used) {
            problems.push(format!(
                "{name}: `needs-no-features: {dep}` is not needed, the recipe does not use features of `{dep}`"
            ));
        }
    }
    problems.sort();
    problems.dedup();
    assert!(problems.is_empty(), "\n{}\n", problems.join("\n"));
}

/// The chips of a recipe copy the features its banner names, not just the crate. This renders every
/// page of the sidebar and checks that, which catches a banner placed in a different section than
/// the chips it belongs to.
#[test]
fn chips_copy_the_features_of_their_banner() {
    let book = Book::load();
    let mut problems = Vec::new();
    for entry in &book.sidebar {
        let page = expand(&project_root().join("src").join(&entry.path));
        let banners = banner_features(&page);
        if banners.is_empty() {
            continue;
        }
        let meta = Meta::for_crates(banners.keys().map(String::as_str));
        let html = render_chapter(&page, &meta);
        let copied: Vec<String> = html
            .split("data-copy=\"")
            .skip(1)
            .filter_map(|part| part.split('"').next())
            .map(|line| line.replace("&quot;", "\""))
            .collect();
        for (krate, features) in &banners {
            let lines: Vec<&String> = copied
                .iter()
                .filter(|line| line.starts_with(&format!("{krate} = ")))
                .collect();
            let has_all =
                |line: &&String| features.iter().all(|f| line.contains(&format!("\"{f}\"")));
            if !lines.is_empty() && !lines.iter().any(has_all) {
                problems.push(format!(
                    "{}: no chip for `{krate}` copies its features ({}); the banner and the chips must be in the same section, or the banner above them",
                    entry.path,
                    features.join(", ")
                ));
            }
        }
    }
    assert!(problems.is_empty(), "\n{}\n", problems.join("\n"));
}
