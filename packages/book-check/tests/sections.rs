//! Every recipe is listed in its section's table, in the order of the sidebar.

use book_check::book::Book;
use book_check::markdown::{expand, headings, is_marked_not_a_recipe, page_headings, NOT_A_RECIPE};
use book_check::{assert_no_problems, project_root};
use std::collections::HashSet;
use std::fs;

#[test]
fn every_row_links_to_a_page_and_heading_that_exist() {
    let book = Book::load();
    let mut problems = Vec::new();
    for (_, section) in book.recipe_sections() {
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
    assert_no_problems(&problems);
}

#[test]
fn every_recipe_is_listed_in_its_sections_table() {
    let book = Book::load();
    let mut problems = Vec::new();
    for (section_index, section) in book.recipe_sections() {
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
    assert_no_problems(&problems);
}

#[test]
fn table_rows_follow_the_sidebar_and_the_recipe_pages() {
    let book = Book::load();
    let mut problems = Vec::new();
    for (_, section) in book.recipe_sections() {
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
    assert_no_problems(&problems);
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
