# SQLx

[`sqlx`][sqlx] is an async SQL toolkit for Rust. It supports connection pools, prepared queries, transactions, and compile-time
checked queries for supported databases. To use [`sqlx`][sqlx] in your application, add the following crates to your project:

```shell
cargo add tokio --features full
cargo add sqlx --features sqlite,postgres,runtime-tokio
```

{{#include sqlx/sqlite_pool.md}}

{{#include sqlx/postgres_pool_typed_rows.md}}

{{#include sqlx/query_macros.md}}

{{#include sqlx/transactions.md}}

{{#include ../links.md}}
