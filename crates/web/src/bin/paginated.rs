use web::paginated;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for dep in paginated::ReverseDependencies::of("serde")? {
        let dependency = dep?;
        println!("{} depends on {}", dependency.id, dependency.crate_id);
    }
    Ok(())
}
