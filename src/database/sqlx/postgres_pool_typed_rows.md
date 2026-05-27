## Connect to Postgres and query typed rows

[![sqlx-badge]][sqlx] [![cat-database-badge]][cat-database]

Use [`PgPool::connect`] to open a connection pool for an existing `locations` Postgres
database. Query values in the database and map query results into a typed `Location` struct with
[`sqlx::query_as`] and [`FromRow`], then load all rows with
[`QueryAs::fetch_all`].

```rust,no_run
{{#include ../../../crates/database/sqlx/src/bin/typed_rows.rs }}
```

[`PgPool::connect`]: https://docs.rs/sqlx/*/sqlx/struct.Pool.html#method.connect
[`sqlx::query_as`]: https://docs.rs/sqlx/*/sqlx/fn.query_as.html
[`FromRow`]: https://docs.rs/sqlx/*/sqlx/trait.FromRow.html
[`QueryAs::fetch_all`]: https://docs.rs/sqlx/*/sqlx/query/struct.QueryAs.html#method.fetch_all
