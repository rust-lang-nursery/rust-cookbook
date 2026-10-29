use rusqlite::{Connection, params};

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute("CREATE TABLE cat (name TEXT, color TEXT)", [])?;
    conn.execute("INSERT INTO cat VALUES (?1, ?2)", params!["Tigger", "Blue"])?;
    conn.execute("INSERT INTO cat VALUES (?1, ?2)", params!["Oreo", "Black"])?;

    let mut stmt = conn.prepare("SELECT name, color FROM cat")?;
    let cats = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    for cat in cats {
        let (name, color) = cat?;
        println!("{name} is {color}");
    }
    Ok(())
}
