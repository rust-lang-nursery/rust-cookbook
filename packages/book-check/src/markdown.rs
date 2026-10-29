//! Reading the book's markdown the way mdBook does.

use crate::project_root;
use mdbook_chips::search_index::badges;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

/// Put this on the line below a heading that is a guide or an explanation rather than a recipe, so
/// the recipe tests skip it.
pub const NOT_A_RECIPE: &str = "<!-- not-a-recipe -->";

pub struct Heading {
    pub level: usize,
    pub text: String,
    pub id: String,
    /// Byte offset in the markdown just after the heading.
    pub end: usize,
}

/// A markdown file with its `{{#include file.md}}` lines replaced by the included text.
pub fn expand(path: &Path) -> String {
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
pub fn headings(markdown: &str) -> Vec<Heading> {
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
pub fn is_marked_not_a_recipe(markdown: &str, heading: &Heading) -> bool {
    markdown[heading.end..]
        .lines()
        .find(|l| !l.trim().is_empty())
        .is_some_and(|l| l.trim() == NOT_A_RECIPE)
}

/// The headings of a page of the sidebar, with its includes expanded.
pub fn page_headings(page: &str) -> Vec<Heading> {
    headings(&expand(&project_root().join("src").join(page)))
}

/// The badge labels on the line under a heading (`[![clap-badge]][clap] [![cat-...]]`), or none if
/// the first thing under it is not a line of badges.
pub fn badges_under(markdown: &str, heading: &Heading) -> BTreeSet<String> {
    markdown[heading.end..]
        .lines()
        .find(|l| !l.trim().is_empty())
        .map(|l| badges(l).into_iter().collect())
        .unwrap_or_default()
}

/// Every markdown file below `dir`.
pub fn markdown_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            markdown_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "md") {
            out.push(path);
        }
    }
}
