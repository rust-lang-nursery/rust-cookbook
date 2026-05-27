## Transactions with SQLx

[![sqlx-badge]][sqlx] [![cat-database-badge]][cat-database]

Use [`PgPoolOptions::connect`] to open a connection pool for an existing
`locations` Postgres database. Start a transaction with [`Pool::begin`], run
two `INSERT` statements with [`sqlx::query`], and save both rows together with
[`Transaction::commit`].

```rust,no_run
{{#include ../../../crates/database/sqlx/src/bin/transactions.rs }}
```

[`PgPoolOptions::connect`]: https://docs.rs/sqlx/*/sqlx/postgres/type.PgPoolOptions.html#method.connect
[`Pool::begin`]: https://docs.rs/sqlx/*/sqlx/struct.Pool.html#method.begin
[`sqlx::query`]: https://docs.rs/sqlx/*/sqlx/fn.query.html
[`Transaction::commit`]: https://docs.rs/sqlx/*/sqlx/struct.Transaction.html#method.commit
