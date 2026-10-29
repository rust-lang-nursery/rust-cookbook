//! The sidebar (`SUMMARY.md`) and the recipe tables of the section pages.

use crate::project_root;
use mdbook_chips::search_index::{badges, parse_link, parse_link_definition};
use std::collections::{BTreeSet, HashMap};
use std::fs;

pub struct Entry {
    pub depth: usize,
    pub path: String,
}

pub struct Row {
    pub title: String,
    pub page: String,
    pub fragment: Option<String>,
    /// The badge labels (`clap`, `cat-command-line`) in the row's Crates and Categories cells.
    pub badges: BTreeSet<String>,
}

pub struct Book {
    pub sidebar: Vec<Entry>,
    links: HashMap<String, String>,
}

impl Book {
    pub fn load() -> Book {
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
    pub fn sections(&self) -> impl Iterator<Item = (usize, &Entry)> {
        self.sidebar
            .iter()
            .enumerate()
            .filter(|(_, e)| e.depth == 0)
    }

    /// The pages below a section in the sidebar, in order, as indexes into `sidebar`.
    pub fn descendants(&self, section: usize) -> Vec<usize> {
        (section + 1..self.sidebar.len())
            .take_while(|&i| self.sidebar[i].depth > 0)
            .collect()
    }

    pub fn is_leaf(&self, i: usize) -> bool {
        self.sidebar
            .get(i + 1)
            .is_none_or(|next| next.depth <= self.sidebar[i].depth)
    }

    pub fn position(&self, page: &str) -> Option<usize> {
        self.sidebar.iter().position(|e| e.path == page)
    }

    /// Section pages: the top-level pages whose table lists recipes.
    pub fn recipe_sections(&self) -> Vec<(usize, String)> {
        self.sections()
            .filter(|(_, e)| {
                fs::read_to_string(project_root().join("src").join(&e.path))
                    .is_ok_and(|t| t.lines().any(|l| l.starts_with("| Recipe")))
            })
            .map(|(i, e)| (i, e.path.clone()))
            .collect()
    }

    /// The recipe rows of a section page, in file order, with the page and heading each links to.
    pub fn rows(&self, section: &str) -> Vec<Row> {
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
