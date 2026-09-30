use rkyv::rancor::Error;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
struct Player {
    name: String,
    level: u32,
    scores: Vec<u32>,
}

fn main() -> Result<(), Error> {
    let player = Player {
        name: "Ferris".to_string(),
        level: 42,
        scores: vec![100, 250, 75],
    };

    let bytes = rkyv::to_bytes::<Error>(&player)?;

    // Validate the buffer and view it as an `ArchivedPlayer` without copying.
    let archived = rkyv::access::<ArchivedPlayer, Error>(&bytes)?;
    println!("name:  {}", archived.name.as_str());
    println!("level: {}", archived.level);
    let total: u32 = archived.scores.iter().map(|s| s.to_native()).sum();
    println!("total: {total}");

    // Convert back into an owned `Player` only when needed.
    let owned = rkyv::deserialize::<Player, Error>(archived)?;
    assert_eq!(owned, player);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn runs() -> Result<(), rkyv::rancor::Error> {
        super::main()
    }
}
