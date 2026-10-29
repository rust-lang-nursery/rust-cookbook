//! Tests on the book's own structure: every recipe is listed in its section's table, in the order
//! of the sidebar and of the recipe pages. They read `src/` the way mdBook does (expanding
//! `{{#include}}`, numbering headings the same way), so they need no build.

use crate::project_root;
use crate::search_index::{parse_link, parse_link_definition};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

/// Sections of a recipe page that are explanations or variants of a recipe, not recipes of their
/// own. Each is `(page, heading)`; keep this list short and say why.
const NOT_RECIPES: &[(&str, &str)] = &[
    // Comparison guides that sit on the page of the first library they discuss.
    (
        "cryptography/hashing.md",
        "Choosing between ring and RustCrypto",
    ),
    (
        "datetime/duration.md",
        "Choosing between chrono, time and jiff",
    ),
    // Language headings that group the C and C++ recipes below them.
    ("development_tools/build_tools.md", "C"),
    ("development_tools/build_tools.md", "C++"),
    // The same recipe written with the tokio executor.
    ("web/clients/requests.md", "Async"),
];

struct Entry {
    depth: usize,
    path: String,
}

struct Heading {
    level: usize,
    text: String,
    id: String,
}

struct Row {
    title: String,
    page: String,
    fragment: Option<String>,
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
            let first = line.trim_matches('|').split('|').next().unwrap_or("");
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
    for event in Parser::new_ext(markdown, options) {
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
                    out.push(Heading { level, text, id });
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
            let mut above: Vec<(usize, bool)> = Vec::new();
            for h in page_headings(page) {
                above.retain(|(level, _)| *level < h.level);
                let own = listed.contains(&(page, h.id.as_str()));
                let covered = own || above.iter().any(|(_, listed)| *listed);
                above.push((h.level, own));
                if h.level >= 2 && !covered && !NOT_RECIPES.contains(&(page, h.text.as_str())) {
                    problems.push(format!(
                        "{section}: no row for \"{}\" (#{}) on {page}",
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
