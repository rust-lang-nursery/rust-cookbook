use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let mut conn = Connection::open("cats.db")?;

    successful_tx(&mut conn)?;

    let res = rolled_back_tx(&mut conn);
    assert!(res.is_err());

    Ok(())
}

fn successful_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from cat_colors", [])?;
    tx.execute("insert into cat_colors (name) values (?1)", ["lavender"])?;
    tx.execute("insert into cat_colors (name) values (?1)", ["blue"])?;

    tx.commit()
}

fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from cat_colors", [])?;
    tx.execute("insert into cat_colors (name) values (?1)", ["lavender"])?;
    tx.execute("insert into cat_colors (name) values (?1)", ["blue"])?;
    tx.execute("insert into cat_colors (name) values (?1)", ["lavender"])?;

    tx.commit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rollback_keeps_committed_rows() -> Result<()> {
        let mut conn = Connection::open_in_memory()?;
        conn.execute(
            "create table cat_colors (id integer primary key, name text not null unique)",
            (),
        )?;

        successful_tx(&mut conn)?;
        assert!(rolled_back_tx(&mut conn).is_err());

        let count: i64 = conn.query_row("select count(*) from cat_colors", [], |r| r.get(0))?;
        assert_eq!(count, 2);
        Ok(())
    }
}
