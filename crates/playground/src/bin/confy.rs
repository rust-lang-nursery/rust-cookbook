use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
struct Prefs {
    username: String,
    autosave: bool,
}

fn main() -> Result<(), confy::ConfyError> {
    let path = std::env::temp_dir().join("playground-prefs.toml");
    let mut prefs: Prefs = confy::load_path(&path)?;
    println!("loaded {prefs:?}");

    prefs.username = "ferris".to_owned();
    confy::store_path(&path, &prefs)?;
    println!("saved to {}", path.display());
    Ok(())
}
