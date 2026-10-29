use config::Config;

fn main() -> Result<(), config::ConfigError> {
    let settings = Config::builder()
        .set_default("host", "127.0.0.1")?
        .set_default("port", 8080)?
        .set_override("port", 9090)?
        .build()?;

    println!(
        "{}:{}",
        settings.get_string("host")?,
        settings.get_int("port")?
    );
    Ok(())
}
