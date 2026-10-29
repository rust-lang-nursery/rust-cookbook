//! A recipe whose code needs a Cargo feature says so in a banner (see CONTRIBUTING.md): when the
//! crate it includes enables features of a dependency the code uses, the recipe's file must have a
//! banner for that dependency, and the banner may only name features the crate really enables.

use book_check::features::{
    dependencies_with_features, included_crate_sources, including_pages, mentions,
    needs_no_features,
};
use book_check::markdown::markdown_files;
use book_check::{assert_no_problems, project_root};
use mdbook_chips::chips::banner_features;
use std::collections::{HashMap, HashSet};
use std::fs;

#[test]
fn recipes_needing_cargo_features_have_a_banner() {
    let src = project_root().join("src");
    let mut files = Vec::new();
    markdown_files(&src, &mut files);
    files.sort();
    let parents = including_pages(&files);
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
    assert_no_problems(&problems);
}
