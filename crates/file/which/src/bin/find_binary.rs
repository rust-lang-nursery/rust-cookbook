use which::{which, which_all};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cargo = which("cargo")?;
    println!("cargo resolves to {}", cargo.display());
    assert!(cargo.is_absolute());

    for path in which_all("cargo")? {
        println!("candidate: {}", path.display());
    }

    match which("definitely-not-a-real-binary") {
        Ok(path) => println!("found at {}", path.display()),
        Err(error) => println!("not on PATH: {error}"),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_cargo_to_an_absolute_path() -> Result<(), which::Error> {
        let cargo = which("cargo")?;
        assert!(cargo.is_absolute());
        Ok(())
    }

    #[test]
    fn missing_binary_is_an_error() {
        assert!(which("definitely-not-a-real-binary").is_err());
    }
}
