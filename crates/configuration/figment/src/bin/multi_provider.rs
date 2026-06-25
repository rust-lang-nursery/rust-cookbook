use figment::providers::{Env, Format, Toml};
use figment::Figment;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    name: String,
    port: u16,
    workers: usize,
}

fn figment() -> Figment {
    Figment::new()
        .merge(Toml::file("App.toml"))
        .merge(Env::prefixed("APP_"))
}

fn main() -> Result<(), figment::Error> {
    let config: Config = figment().extract()?;
    println!(
        "{} listening on port {} with {} workers",
        config.name, config.port, config.workers
    );
    Ok(())
}

#[test]
fn env_overrides_the_toml_file() {
    use figment::Jail;

    Jail::expect_with(|jail| {
        jail.create_file(
            "App.toml",
            r#"
                name = "api"
                port = 8000
                workers = 4
            "#,
        )?;
        jail.set_env("APP_PORT", 9000);

        let config: Config = figment().extract()?;
        assert_eq!(config.name, "api");
        assert_eq!(config.port, 9000);
        assert_eq!(config.workers, 4);
        Ok(())
    });
}
