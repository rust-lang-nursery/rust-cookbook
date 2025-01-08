use crate::project_root;
use std::error::Error;
use std::process::Command;

pub fn run_test(task: &str) -> Result<(), Box<dyn Error>> {
    match task {
        "all" => run_all_tests()?,
        "cargo" => cargo_test()?,
        "spellcheck" => spellcheck()?,
        "link" => link_checker()?,
        _ => run_all_tests()?,
    }
    Ok(())
}

fn run_all_tests() -> Result<(), Box<dyn Error>> {
    let mut failures = Vec::new();

    if cargo_test().is_err() {
        failures.push("cargo_test".to_string());
    }

    if spellcheck().is_err() {
        failures.push("spellcheck".to_string());
    }

    if link_checker().is_err() {
        failures.push("link".to_string());
    }

    if !failures.is_empty() {
        println!("\n--- Test Summary ---");
        for name in failures {
            eprintln!("❌ {name} failed! Re-run with the command:");
            eprintln!("   cargo xtask test {name}");
        }
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "test failed",
        )))
    } else {
        println!("\n🎉 All tests passed!");
        Ok(())
    }
}

fn cargo_test() -> Result<(), Box<dyn Error>> {
    let status = Command::new("cargo")
        .current_dir(project_root())
        .args(["test", "--package", "rust-cookbook"])
        .status()?;

    if !status.success() {
        return Err("failed to run cargo test!".into());
    }

    Ok(())
}

fn spellcheck() -> Result<(), Box<dyn Error>> {
    let status = Command::new("./ci/spellcheck.sh")
        .current_dir(project_root())
        .status()?;

    if !status.success() {
        return Err("failed to run spellcheck!".into());
    }

    Ok(())
}

fn link_checker() -> Result<(), Box<dyn Error>> {
    if Command::new("lychee").arg("--version").status().is_err() {
        return Err(
            "The `lychee` tool is not installed. Please install it using:\n    cargo install lychee@0.17.0".into(),
        );
    }

    let book_dir = project_root().join("book");
    if !book_dir.is_dir() {
        return Err(format!(
            "The book directory could not be found in the root directory: {:?}\n\
            You can build it using:\n    cargo xtask book build",
            book_dir
        )
        .into());
    }

    let status = Command::new("lychee")
        .current_dir(project_root())
        .args([
            "./book",
            "--retry-wait-time",
            "20",
            "--max-retries",
            "3",
            "--accept",
            "429", // accept 429 (ratelimit) errors as valid
        ])
        .status()?;

    if !status.success() {
        return Err("Failed to run link checker!".into());
    }

    Ok(())
}
