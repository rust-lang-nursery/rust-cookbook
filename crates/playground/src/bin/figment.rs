use figment::Figment;
use figment::providers::Serialized;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    name: String,
    port: u16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Later providers override earlier ones; real apps merge Toml::file(..) and Env::prefixed(..).
    let config: Config = Figment::from(Serialized::defaults(Config {
        name: "api".to_owned(),
        port: 8000,
    }))
    .merge(Serialized::default("port", 9000))
    .extract()?;

    println!("{config:?}");
    Ok(())
}
