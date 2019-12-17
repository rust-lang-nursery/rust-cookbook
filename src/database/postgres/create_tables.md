## Create tables in a Postgres database

[![postgres-badge]][postgres] [![cat-database-badge]][cat-database]

Use the [`postgres`] crate to create tables in a Postgres database.

[`Connection::connect`] connects to an existing database. The recipe uses a URL string format with `Connection::connect`.
The recipe requires an existing database named `library`, with a `postgres` user with `postgres` password.

```rust,no_run
extern crate postgres;

use postgres::{Connection, TlsMode, Error};

fn main() -> Result<(), Error> {
    let conn = Connection::connect("postgresql://postgres:postgres@localhost/library", 
                                    TlsMode::None)?;
    
     conn.execute("CREATE TABLE IF NOT EXISTS author (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    country         VARCHAR NOT NULL
                  )", &[])?;

    conn.execute("CREATE TABLE IF NOT EXISTS book  (
                    id              SERIAL PRIMARY KEY,
                    title           VARCHAR NOT NULL,
                    author_id       INTEGER NOT NULL REFERENCES author
                )", &[])?;

    Ok(())

}
```

[`postgres`]: https://docs.rs/postgres/0.15.2/postgres/
[`Connection::connect`]: https://docs.rs/postgres/0.15.2/postgres/struct.Connection.html#method.connect
