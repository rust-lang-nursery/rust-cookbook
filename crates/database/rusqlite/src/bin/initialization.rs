use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("cats.db")?;
    create_tables(&conn)?;

    Ok(())
}

fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
        (),
    )?;
    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             color_id integer not null references cat_colors(id)
         )",
        (),
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_tables_idempotently() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        create_tables(&conn)?;
        create_tables(&conn)?;

        let count: i64 = conn.query_row(
            "select count(*) from sqlite_master where type = 'table' and name in ('cats', 'cat_colors')",
            [],
            |row| row.get(0),
        )?;
        assert_eq!(count, 2);
        Ok(())
    }
}
