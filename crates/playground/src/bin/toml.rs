use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    version: String,
}

fn main() -> Result<(), toml::de::Error> {
    let package: Package = toml::from_str(
        r#"
        name = "cookbook"
        version = "1.0.0"
        "#,
    )?;
    println!("{} {}", package.name, package.version);
    Ok(())
}
