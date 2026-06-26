use std::path::Path;

use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Settings {
    host: String,
    port: u16,
    log_level: String,
}

fn load(config_path: &Path) -> Result<Settings, config::ConfigError> {
    Config::builder()
        .set_default("host", "127.0.0.1")?
        .set_default("port", 8080)?
        .set_default("log_level", "info")?
        .add_source(File::from(config_path).required(false))
        .add_source(Environment::with_prefix("APP"))
        .build()?
        .try_deserialize()
}

fn main() -> Result<(), config::ConfigError> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("config.toml");
    let settings = load(&path)?;
    println!("listening on {}:{}", settings.host, settings.port);
    println!("log level: {}", settings.log_level);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_overrides_defaults() -> Result<(), config::ConfigError> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("config.toml");
        let settings = load(&path)?;
        assert_eq!(settings.host, "0.0.0.0");
        assert_eq!(settings.port, 3000);
        assert_eq!(settings.log_level, "debug");
        Ok(())
    }

    #[test]
    fn defaults_apply_when_file_is_missing() -> Result<(), config::ConfigError> {
        let settings = load(Path::new("does-not-exist.toml"))?;
        assert_eq!(settings.host, "127.0.0.1");
        assert_eq!(settings.port, 8080);
        Ok(())
    }
}
