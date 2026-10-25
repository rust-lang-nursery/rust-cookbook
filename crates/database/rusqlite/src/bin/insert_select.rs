use rusqlite::{Connection, Result, params};
use std::collections::HashMap;

#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
}

fn main() -> Result<()> {
    let conn = Connection::open("cats.db")?;

    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    insert_cats(&conn, &cat_colors)?;

    for found_cat in select_cats(&conn)? {
        println!(
            "Found cat {:?} {} is {}",
            found_cat, found_cat.name, found_cat.color,
        );
    }

    Ok(())
}

fn insert_cats(conn: &Connection, cat_colors: &HashMap<String, Vec<&str>>) -> Result<()> {
    for (color, catnames) in cat_colors {
        conn.execute("INSERT INTO cat_colors (name) VALUES (?1)", [color])?;
        let last_id = conn.last_insert_rowid();

        for cat in catnames {
            conn.execute(
                "INSERT INTO cats (name, color_id) values (?1, ?2)",
                params![cat, last_id],
            )?;
        }
    }

    Ok(())
}

fn select_cats(conn: &Connection) -> Result<Vec<Cat>> {
    let mut stmt = conn.prepare(
        "SELECT c.name, cc.name FROM cats c
         INNER JOIN cat_colors cc
         ON cc.id = c.color_id;",
    )?;

    let cats = stmt.query_map([], |row| {
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;

    cats.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserts_and_selects_cats() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(
            "create table cat_colors (id integer primary key, name text not null unique);
             create table cats (
                 id integer primary key,
                 name text not null,
                 color_id integer not null references cat_colors(id)
             );",
        )?;

        let mut cat_colors = HashMap::new();
        cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
        insert_cats(&conn, &cat_colors)?;

        let mut cats = select_cats(&conn)?;
        cats.sort_by(|a, b| a.name.cmp(&b.name));
        let found: Vec<_> = cats.iter().map(|c| (&*c.name, &*c.color)).collect();
        assert_eq!(found, [("Sammy", "Blue"), ("Tigger", "Blue")]);
        Ok(())
    }
}
