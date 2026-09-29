//! Formats the Rust code blocks that live inline in the book's markdown.
//!
//! The skeptic-tested examples (see `build.rs`) are written directly in
//! `src/**/*.md`, so `cargo fmt` never sees them.  This task extracts each
//! block, runs `rustfmt` over it, and either rewrites the markdown or reports
//! the difference.
//!
//! Examples that have been migrated to standalone crates are pulled in with
//! `{{#include}}` and are already covered by `cargo fmt --all`.

use crate::project_root;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A code block we decided not to touch, and why.
struct Skipped {
    file: String,
    line: usize,
    reason: &'static str,
}

pub fn run_fmt_md(check: bool) -> Result<(), Box<dyn Error>> {
    let root = project_root();
    let mut files = Vec::new();
    collect_markdown(&root.join("src"), &mut files)?;
    files.sort();

    let mut reformatted = Vec::new();
    let mut skipped = Vec::new();
    let mut scanned = 0usize;

    for path in &files {
        let original = fs::read_to_string(path)?;
        let rel = path
            .strip_prefix(&root)
            .unwrap_or(path)
            .display()
            .to_string();

        let (rewritten, changed) = format_markdown(
            &original,
            &rel,
            &mut scanned,
            &mut reformatted,
            &mut skipped,
        )?;

        if changed && !check {
            fs::write(path, rewritten)?;
        }
    }

    report(scanned, &reformatted, &skipped, check);

    if check && !reformatted.is_empty() {
        return Err("inline markdown code is not formatted".into());
    }
    Ok(())
}

/// Walks the book source, rewriting every eligible code block.
///
/// Returns the new file contents and whether anything actually changed.
fn format_markdown(
    source: &str,
    rel: &str,
    scanned: &mut usize,
    reformatted: &mut Vec<String>,
    skipped: &mut Vec<Skipped>,
) -> Result<(String, bool), Box<dyn Error>> {
    // Part of the book is checked in with CRLF endings; rustfmt always emits
    // LF, so remember which terminator this file uses and put it back.
    let newline = if source.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };

    let lines: Vec<&str> = source.lines().collect();
    let mut out: Vec<String> = Vec::with_capacity(lines.len());
    let mut changed = false;
    let mut i = 0;

    while i < lines.len() {
        let Some(info) = lines[i].strip_prefix("```rust") else {
            out.push(lines[i].to_string());
            i += 1;
            continue;
        };

        // Find the closing fence.
        let mut end = i + 1;
        while end < lines.len() && !lines[end].starts_with("```") {
            end += 1;
        }
        if end == lines.len() {
            // Unterminated fence; leave the file alone.
            out.push(lines[i].to_string());
            i += 1;
            continue;
        }

        let body = &lines[i + 1..end];
        let fence_line = i + 1;
        *scanned += 1;

        match skip_reason(info, body) {
            Some(reason) => skipped.push(Skipped {
                file: rel.to_string(),
                line: fence_line,
                reason,
            }),
            None => {
                let code = format!("{}\n", body.join("\n"));
                match rustfmt(&code, edition(info))? {
                    Some(formatted) if formatted != code => {
                        reformatted.push(format!("{rel}:{fence_line}"));
                        changed = true;
                        out.push(lines[i].to_string());
                        out.extend(formatted.lines().map(str::to_string));
                        out.push(lines[end].to_string());
                        i = end + 1;
                        continue;
                    }
                    Some(_) => {}
                    None => skipped.push(Skipped {
                        file: rel.to_string(),
                        line: fence_line,
                        reason: "rustfmt could not parse the block",
                    }),
                }
            }
        }

        out.extend(lines[i..=end].iter().map(|l| l.to_string()));
        i = end + 1;
    }

    let mut rewritten = out.join(newline);
    if source.ends_with('\n') {
        rewritten.push_str(newline);
    }
    Ok((rewritten, changed))
}

/// Blocks we deliberately leave alone.
fn skip_reason(info: &str, body: &[&str]) -> Option<&'static str> {
    if info.contains("ignore") {
        // Not compiled by skeptic, so it need not be valid Rust.
        return Some("`ignore` block");
    }
    if body.iter().any(|l| l.contains("{{#include")) {
        // Lives in a workspace crate; `cargo fmt --all` already covers it.
        return Some("`{{#include}}` of a workspace crate");
    }
    if body.iter().any(|l| is_hidden(l)) {
        // Skeptic hides `# ` lines from the rendered book but still compiles
        // them.  rustfmt reflows long lines, which would move code across the
        // hidden/visible boundary and silently change what readers see, so
        // these are left for a human.
        return Some("contains skeptic `# ` hidden lines");
    }
    None
}

/// True for a skeptic hidden line (`#` or `# ...`), but not an attribute.
fn is_hidden(line: &str) -> bool {
    line == "#" || (line.starts_with("# ") && !line.starts_with("# ["))
}

/// Picks the edition from the fence annotation, matching `book.toml`'s default.
fn edition(info: &str) -> &'static str {
    if info.contains("edition2024") {
        "2024"
    } else if info.contains("edition2021") {
        "2021"
    } else if info.contains("edition2015") {
        "2015"
    } else {
        "2018"
    }
}

/// Runs rustfmt over a snippet, returning `None` if it could not be parsed.
fn rustfmt(code: &str, edition: &str) -> Result<Option<String>, Box<dyn Error>> {
    let mut path = std::env::temp_dir();
    path.push(format!("cookbook-fmt-{}.rs", std::process::id()));
    fs::write(&path, code)?;

    let status = Command::new("rustfmt")
        .args(["--edition", edition, "--emit", "files"])
        .arg(&path)
        .status()?;

    let formatted = fs::read_to_string(&path)?;
    fs::remove_file(&path).ok();

    Ok(status.success().then_some(formatted))
}

fn collect_markdown(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_markdown(&path, out)?;
        } else if path.extension().is_some_and(|e| e == "md") {
            out.push(path);
        }
    }
    Ok(())
}

fn report(scanned: usize, reformatted: &[String], skipped: &[Skipped], check: bool) {
    println!("\n--- Inline markdown formatting ---");
    println!("scanned {scanned} code blocks");

    if !skipped.is_empty() {
        println!("\nskipped {} block(s):", skipped.len());
        for s in skipped {
            println!("  {}:{} — {}", s.file, s.line, s.reason);
        }
    }

    if reformatted.is_empty() {
        println!("\n✅ All inline code blocks are formatted.");
        return;
    }

    let verb = if check {
        "need formatting"
    } else {
        "reformatted"
    };
    println!("\n{} block(s) {verb}:", reformatted.len());
    for r in reformatted {
        println!("  {r}");
    }
    if check {
        eprintln!("\n❌ Run `cargo xtask fmt-md` to fix them.");
    }
}
