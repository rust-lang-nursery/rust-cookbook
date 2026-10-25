## Insert and Select data

[![rusqlite-badge]][rusqlite] [![cat-database-badge]][cat-database]

[`Connection::open`] will open the database `cats` created in the earlier recipe.
This recipe inserts data into `cat_colors` and `cats` tables using the [`execute`] method of `Connection`. First, the data is inserted into the `cat_colors` table. After a record for a color is inserted, [`last_insert_rowid`] method of `Connection` is used to get `id` of the last color inserted. This `id` is used while inserting data into the `cats` table. Then, the select query is prepared using the [`prepare`] method which gives a [`statement`] struct. Then, query is executed using [`query_map`] method of [`statement`].

```rust,no_run
{{#include ../../../crates/database/rusqlite/src/bin/insert_select.rs::60 }}
```

[`Connection::open`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.open
[`prepare`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.prepare
[`statement`]: https://docs.rs/rusqlite/*/rusqlite/struct.Statement.html
[`query_map`]: https://docs.rs/rusqlite/*/rusqlite/struct.Statement.html#method.query_map
[`execute`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.execute
[`last_insert_rowid`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.last_insert_rowid
