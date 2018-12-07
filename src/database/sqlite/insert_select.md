## Insert and Select data

[![rusqlite-badge]][rusqlite] [![cat-database-badge]][cat-database]

[`Connection::open`] will open the database `cats` created in the earlier recipe.
This recipe inserts data into `cat_colors` and `cats` tables using the [`execute`] method of `Connection`. First, the data is inserted into the `cat_colors` table. After a record for a color is inserted, [`last_insert_rowid`] method of `Connection` is used to get `id` of the last color inserted. This `id` is used while inserting data into the `cats` table. Then, the select query is prepared using the [`prepare`] method which gives a [`statement`] struct. Then, query is executed using [`query_map`] method of [`statement`].

```
extern crate chrono;
extern crate rusqlite;

use rusqlite::{Connection, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::collections::HashMap;

#[derive(Debug)]
struct Cat {
    name: String,
    date_of_birth: DateTime<Utc>,
    color: String
}

fn main() -> Result<()> {
    let conn = Connection::open("cats.db")?;
    let date_of_birth = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);

    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    for (color, catnames) in &cat_colors{
        conn.execute(
            "INSERT INTO cat_colors (name) values (?1)",
            &[&color.to_string()],
        )?;
    let last_id = conn.last_insert_rowid();
    for cat in catnames{
        conn.execute(
            "INSERT INTO cats (name, date_of_birth, color_id) values (?1, ?2, ?3)",
            &[&cat.to_string(), &date_of_birth, &last_id],
        )?;
        }
    }
    let mut stmt = conn.prepare("SELECT c.name, date_of_birth, cc.name from cats c 
                                 INNER JOIN cat_colors cc ON cc.id = c.color_id;")?;
    let cats = stmt.query_map(&[], |row| {
        Cat {
            name: row.get(0),
            date_of_birth: row.get(1),
            color: row.get(2)
        }
    })?;
    
    for cat in cats {
        println!("Found cat {:?}", cat);
    }

    Ok(())
}

```

[`Connection::open`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.open
[`prepare`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.prepare
[`statement`]: https://docs.rs/rusqlite/*/rusqlite/struct.Statement.html
[`query_map`]: https://docs.rs/rusqlite/*/rusqlite/struct.Statement.html#method.query_map
[`execute`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.execute
[`last_insert_rowid`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.last_insert_rowid
