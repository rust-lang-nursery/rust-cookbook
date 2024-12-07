mod paginated;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_dependencies() -> Result<()> {
        for dep in paginated::ReverseDependencies::of("serde")?.take(5) {
            let dependency = dep?;
            println!("{} depends on {}", dependency.id, dependency.crate_id);
        }
        Ok(())
    }
}
