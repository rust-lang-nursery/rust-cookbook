//! The chips of a recipe match its table row, and copy the features of its banner.

use book_check::book::Book;
use book_check::markdown::{badges_under, expand, headings};
use book_check::{assert_no_problems, project_root};
use mdbook_chips::chips::{banner_features, render_chapter, Meta};
use std::collections::BTreeSet;

#[test]
fn every_recipe_shows_the_same_chips_as_its_table_row() {
    let book = Book::load();
    let mut problems = Vec::new();
    for (_, section) in book.recipe_sections() {
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
    assert_no_problems(&problems);
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
    assert_no_problems(&problems);
}
