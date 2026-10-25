## Create a SQLite database

[![rusqlite-badge]][rusqlite] [![cat-database-badge]][cat-database]

Use the `rusqlite` crate to open SQLite databases. See
[crate][documentation] for compiling on Windows.

[`Connection::open`] will create the database if it doesn't already exist.

```rust,no_run
{{#include ../../../crates/database/rusqlite/src/bin/initialization.rs::28 }}
```

[`Connection::open`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.open

[documentation]: https://github.com/rusqlite/rusqlite#user-content-notes-on-building-rusqlite-and-libsqlite3-sys
