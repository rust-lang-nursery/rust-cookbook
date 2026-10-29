use postgres::Config;

fn main() -> Result<(), postgres::Error> {
    // The playground has no database; this parses a connection string without connecting.
    let config: Config = "host=localhost user=postgres dbname=cookbook".parse()?;
    println!("{config:?}");
    // let mut client = config.connect(postgres::NoTls)?;
    Ok(())
}
