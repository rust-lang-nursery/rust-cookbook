use anyhow::{Context, Result};

fn main() -> Result<()> {
    let n: i32 = "42".parse().context("not a number")?;
    println!("parsed {n}");
    Ok(())
}
