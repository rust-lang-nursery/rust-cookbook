use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Prefs {
    username: String,
    theme: String,
    autosave: bool,
}

impl Default for Prefs {
    fn default() -> Self {
        Prefs {
            username: "anonymous".to_owned(),
            theme: "dark".to_owned(),
            autosave: true,
        }
    }
}

fn main() -> Result<(), confy::ConfyError> {
    let path = confy::get_configuration_file_path("cookbook-prefs", None)?;
    println!("per-user config path: {}", path.display());

    let scratch = std::env::temp_dir().join("cookbook-prefs.toml");
    let mut prefs: Prefs = confy::load_path(&scratch)?;
    println!("loaded {prefs:?}");

    prefs.theme = "light".to_owned();
    confy::store_path(&scratch, &prefs)?;
    println!("saved {prefs:?} to {}", scratch.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_a_file() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("prefs.toml");

        let prefs: Prefs = confy::load_path(&path)?;
        assert_eq!(prefs.theme, "dark");
        assert!(prefs.autosave);

        confy::store_path(
            &path,
            &Prefs {
                theme: "light".to_owned(),
                ..Prefs::default()
            },
        )?;

        let reloaded: Prefs = confy::load_path(&path)?;
        assert_eq!(reloaded.theme, "light");
        Ok(())
    }
}
