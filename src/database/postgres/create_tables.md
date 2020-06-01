## Create tables in a Postgres database

[![postgres-badge]][postgres] [![cat-database-badge]][cat-database]

Use the [`postgres`] crate to create tables in a Postgres database.

[`Client::connect`] helps in connecting to an existing database. The recipe uses a URL string format with `Client::connect`. It assumes an existing database named `library`, the username is `postgres` and the password is `postgres`.

```rust,edition2018,no_run
use postgres::{Client, NoTls, Error};

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;
    
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS author (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            country         VARCHAR NOT NULL
            )
    ")?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS book  (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            author_id       INTEGER NOT NULL REFERENCES author
            )
    ")?;

    Ok(())

}
```

[`postgres`]: https://docs.rs/postgres/0.17.2/postgres/
[`Client::connect`]: https://docs.rs/postgres/0.17.2/postgres/struct.Client.html#method.connect
