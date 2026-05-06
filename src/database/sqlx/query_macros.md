## Compile-time checked queries

[![sqlx-badge]][sqlx] [![cat-database-badge]][cat-database]

Use [`Connection::connect`] to open a Postgres connection and
[`sqlx::query_as!`] to validate the SQL query against the database schema at
compile time. The macro maps the selected columns into a `Location` struct
and fetches a single row using [`Query::fetch_one`].

```rust,ignore
{{#include ../../../crates/database/sqlx/src/bin/query_macros.rs }}
```

Set `DATABASE_URL` to a Postgres database that already contains the `location`
table before building. This may be achieved by either adding a `DATABASE_URL` variable
to a `.env` file or running:

```shell
export DATABASE_URL=<your_database_url>
```
[`Query::fetch_one`]: https://docs.rs/sqlx/latest/sqlx/query/struct.Query.html#method.fetch_one
[`Connection::connect`]: https://docs.rs/sqlx/*/sqlx/trait.Connection.html#tymethod.connect
[`sqlx::query_as!`]: https://docs.rs/sqlx/*/sqlx/macro.query_as.html
