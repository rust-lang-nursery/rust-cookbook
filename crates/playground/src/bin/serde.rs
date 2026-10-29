use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

fn main() -> Result<(), serde_json::Error> {
    // serde defines the derive and traits; a format crate such as serde_json does the encoding.
    let json = serde_json::to_string(&Point { x: 1, y: 2 })?;
    println!("{json}");

    let point: Point = serde_json::from_str(&json)?;
    println!("{point:?}");
    Ok(())
}
