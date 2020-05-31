## Insert and Query data

[![postgres-badge]][postgres] [![cat-database-badge]][cat-database]

The recipe inserts data into the `author` table using [`execute`] method of `Client`. Then, displays the data from the `author` table  using [`query`] method of `Client`.


```rust,edition2018,no_run
use postgres::{Client, NoTls, Error};
use std::collections::HashMap;

struct Author {
    _id: i32,
    name: String,
    country: String
}

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", 
                                    NoTls)?;
    
    let mut authors = HashMap::new();
    authors.insert(String::from("Chinua Achebe"), "Nigeria");
    authors.insert(String::from("Rabindranath Tagore"), "India");
    authors.insert(String::from("Anita Nair"), "India");

    for (key, value) in &authors {
        let author = Author {
            _id: 0,
            name: key.to_string(),
            country: value.to_string()
        };

        client.execute(
                "INSERT INTO author (name, country) VALUES ($1, $2)",
                &[&author.name, &author.country],
        )?;
    }

    for row in client.query("SELECT id, name, country FROM author", &[])? {
        let author = Author {
            _id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };
        println!("Author {} is from {}", author.name, author.country);
    }

    Ok(())

}
```

[`execute`]: https://docs.rs/postgres/0.17.2/postgres/struct.Client.html#method.execute
[`query`]: https://docs.rs/postgres/0.17.2/postgres/struct.Client.html#method.query
