mod paginated;

fn main() -> Result<()> {
    for dep in paginated::ReverseDependencies::of("serde")? {
        let dependency = dep?;
        println!("{} depends on {}", dependency.id, dependency.crate_id);
    }
    Ok(())
}

