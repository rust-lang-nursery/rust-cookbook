## Insert and Query data

[![postgres-badge]][postgres] [![cat-database-badge]][cat-database]

The recipe inserts data into the `author` table using [`execute`] method of `Connection`. Then, displays the data from the `author` table  using [`query`] method of `Connection`.

```rust,no_run
extern crate postgres;

use postgres::{Connection, TlsMode, Error};
use std::collections::HashMap;

struct Author {
    id: i32,
    name: String,
    country: String
}

fn main() -> Result<(), Error> {
    let conn = Connection::connect("postgresql://postgres:postgres@localhost/library", 
                                    TlsMode::None)?;
    
    let mut authors = HashMap::new();
    authors.insert(String::from("Chinua Achebe"), "Nigeria");
    authors.insert(String::from("Rabindranath Tagore"), "India");
    authors.insert(String::from("Anita Nair"), "India");

    for (key, value) in &authors {
        let author = Author {
            id: 0,
            name: key.to_string(),
            country: value.to_string()
        };

        conn.execute("INSERT INTO author (name, country) VALUES ($1, $2)",
                 &[&author.name, &author.country])?;
    }

    for row in &conn.query("SELECT id, name, country FROM author", &[])? {
        let author = Author {
            id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };
        println!("Author {} is from {}", author.name, author.country);
    }

    Ok(())

}
```

[`execute`]: https://docs.rs/postgres/0.15.2/postgres/struct.Connection.html#method.execute
[`query`]: https://docs.rs/postgres/0.15.2/postgres/struct.Connection.html#method.query
