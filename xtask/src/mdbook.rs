use crate::project_root;
use std::{error::Error, path::PathBuf, process::Command};

pub fn run_book(task: &str) -> Result<(), Box<dyn Error>> {
    let args: &[&str] = if task == "serve" { &["--open"] } else { &[] };

    execute_mdbook_command(task, args)?;

    Ok(())
}

fn execute_mdbook_command(command: &str, additional_args: &[&str]) -> Result<(), Box<dyn Error>> {
    check_mdbook_version()?;

    let book_dest = project_root().join("book").to_str().unwrap().to_string();

    let mut args = vec![command, "--dest-dir", &book_dest];
    args.extend_from_slice(additional_args);

    let status = Command::new("mdbook")
        .current_dir(project_root())
        .args(&args)
        .status()?;

    if !status.success() {
        return Err(format!("`mdbook {command}` failed to run successfully!").into());
    }

    Ok(())
}

fn check_mdbook_version() -> Result<(), Box<dyn Error>> {
    if Command::new("mdbook").arg("--version").status().is_err() {
        println!("Error: `mdbook` not found. Please ensure it is installed!");
        println!("You can install it using:");
        println!("    cargo install mdbook");
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "`mdbook` is not installed",
        )));
    }

    Ok(())
}