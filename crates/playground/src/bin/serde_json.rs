use serde_json::{Value, json};

fn main() -> Result<(), serde_json::Error> {
    let value: Value = serde_json::from_str(r#"{"name": "Ferris", "legs": 8}"#)?;
    println!("{} has {} legs", value["name"], value["legs"]);

    let built = json!({ "crate": "serde_json", "tags": ["json", "serde"] });
    println!("{}", serde_json::to_string_pretty(&built)?);
    Ok(())
}
