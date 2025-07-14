## Create a SQLite database

[![rusqlite-badge]][rusqlite] [![cat-database-badge]][cat-database]

Use the `rusqlite` crate to open SQLite databases. See
[crate][documentation] for compiling on Windows.

[`Connection::open`] will create the database if it doesn't already exist.

```rust,edition2021,no_run
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("cats.db")?;

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
```

[`Connection::open`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.open

[documentation]: https://github.com/rusqlite/rusqlite#user-content-notes-on-building-rusqlite-and-libsqlite3-sys
