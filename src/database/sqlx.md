# SQLx

[`sqlx`][sqlx] is an async SQL toolkit for Rust. It supports connection pools, prepared queries, transactions, and compile-time
checked queries for supported databases.
> This recipe requires the [`sqlite`], [`postgres`] and [`runtime-tokio`] feature flags of `sqlx` and the [`full`] feature flag of `tokio` to be enabled in `Cargo.toml`.

Add both crates to your project with:

```shell
cargo add tokio --features full
cargo add sqlx --features sqlite,postgres,runtime-tokio
```

{{#include sqlx/sqlite_pool.md}}

{{#include sqlx/postgres_pool_typed_rows.md}}

{{#include sqlx/query_macros.md}}

{{#include sqlx/transactions.md}}

{{#include ../links.md}}
[`full`]: https://docs.rs/crate/tokio/*/features#full
[`postgres`]: https://docs.rs/crate/sqlx/*/features#postgres
[`runtime-tokio`]: https://docs.rs/crate/sqlx/*/features#runtime-tokio
[`sqlite`]: https://docs.rs/crate/sqlx/*/features#sqlite
