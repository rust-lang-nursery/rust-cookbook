## Insert and Select data

[![rusqlite-badge]][rusqlite] [![cat-database-badge]][cat-database]

[`Connection::open`] will open the database `cats` created in the earlier recipe.
This recipe inserts data into `cat_colors` and `cats` tables using the [`execute`] method of `Connection`. First, the data is inserted into the `cat_colors` table. After a record for a color is inserted, [`last_insert_rowid`] method of `Connection` is used to get `id` of the last color inserted. This `id` is used while inserting data into the `cats` table. Then, the select query is prepared using the [`prepare`] method which gives a [`statement`] struct. Then, query is executed using [`query_map`] method of [`statement`].

```
extern crate chrono;
extern crate rusqlite;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use chrono::{DateTime, Utc, TimeZone};
use std::collections::HashMap;

#[derive(Debug)]
struct Cat {
    name: String,
    date_of_birth: DateTime<Utc>,
    color: String
}

fn main() -> Result<()> {
    let conn = Connection::open("cats.db")?;
    
	let date_of_birth : String = Utc::now().to_rfc3339();

    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    for (color, catnames) in &cat_colors{
        conn.execute(
            "INSERT INTO cat_colors (name) values (?1)",
            &[&color.to_string()],
        )?;
    let last_id : String = conn.last_insert_rowid().to_string();

    for cat in catnames{
        conn.execute(
            "INSERT INTO cats (name, date_of_birth, color_id) values (?1, ?2, ?3)",
            &[&cat.to_string(), &date_of_birth, &last_id],
        )?;
        }
    }
    let mut stmt = conn.prepare("SELECT c.name, date_of_birth, cc.name from cats c 
                                 INNER JOIN cat_colors cc ON cc.id = c.color_id;")?;
    
	let cats = stmt
        .query_map(rusqlite::NO_PARAMS, |row| 
            //Go through each returned row, and check for a valid date
			Ok( if valid_date(row.get(1)?)==false {
					return std::result::Result::Err(rusqlite::Error::InvalidQuery);
				} else { 
				Cat {
					name: row.get(0)?,
					date_of_birth: {
						let dob_string : String = row.get(1)?;
						let dob : i64 = DateTime::parse_from_rfc3339(&dob_string).unwrap().timestamp();
						let new_utc : DateTime<Utc> = Utc.timestamp(dob, 0);
						new_utc
						},
					color: row.get(2)?,
				}
				}
			)
		)?;	
    
    for cat in cats {
        println!("Found cat {:?}", cat);
    }

    Ok(())
}


fn valid_date(incoming_date_string : String) -> bool {	
	let return_value : bool = DateTime::parse_from_rfc3339(&incoming_date_string).is_ok();
	
	return return_value;
}

```

[`Connection::open`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.open
[`prepare`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.prepare
[`statement`]: https://docs.rs/rusqlite/*/rusqlite/struct.Statement.html
[`query_map`]: https://docs.rs/rusqlite/*/rusqlite/struct.Statement.html#method.query_map
[`execute`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.execute
[`last_insert_rowid`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.last_insert_rowid
