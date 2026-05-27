## Connect to SQLite and query data

[![sqlx-badge]][sqlx] [![cat-database-badge]][cat-database]

Use [`SqlitePoolOptions::connect`] to open an in-memory SQLite database backed by a connection pool that only allows a maximum of 1 concurrent connections.
Create a table and insert rows with [`sqlx::query`] and [`Executor::execute`], then load the results with [`Query::fetch_all`].
Each row is read by calling [`Row::try_get`] for the selected columns.

```rust,no_run
{{#include ../../../crates/database/sqlx/src/bin/sqlite_pool.rs::47 }}
```

[`SqlitePoolOptions::connect`]: https://docs.rs/sqlx/*/sqlx/sqlite/type.SqlitePoolOptions.html#method.connect
[`sqlx::query`]: https://docs.rs/sqlx/*/sqlx/fn.query.html
[`Executor::execute`]: https://docs.rs/sqlx/*/sqlx/trait.Executor.html#method.execute
[`Query::fetch_all`]: https://docs.rs/sqlx/*/sqlx/query/struct.Query.html#method.fetch_all
[`Row::try_get`]: https://docs.rs/sqlx/*/sqlx/trait.Row.html#method.try_get
