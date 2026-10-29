## Create, query, and update rows with SeaORM
[![sea_orm-badge]][sea_orm] [![tokio-badge]][tokio] [![cat-database-badge]][cat-database]

Use [`Database::connect`] to open an in-memory SQLite database and
[`Schema::create_table_from_entity`] to create a table from a SeaORM entity.
Create a row with [`ActiveModelTrait::save`], load it with [`EntityTrait::find`],
then update it and check the saved value returned by [`ActiveModelTrait::save`].

> This recipe requires the [`macros`], [`runtime-tokio-native-tls`] and [`sqlx-sqlite`] feature flags of `sea-orm` and the [`macros`] and [`rt-multi-thread`] feature flags of `tokio` to be enabled in `Cargo.toml`.

```rust
{{#include ../../../crates/database/sea_orm/src/bin/simple_orm_pattern.rs::58 }}
```

[`Database::connect`]: https://docs.rs/sea_orm/*/sea_orm/struct.Database.html#method.connect
[`Schema::create_table_from_entity`]: https://docs.rs/sea_orm/*/sea_orm/schema/struct.Schema.html#method.create_table_from_entity
[`ActiveModelTrait::save`]: https://docs.rs/sea_orm/*/sea_orm/entity/trait.ActiveModelTrait.html#method.save
[`EntityTrait::find`]: https://docs.rs/sea_orm/*/sea_orm/entity/trait.EntityTrait.html#tymethod.find
[`macros`]: https://docs.rs/crate/sea-orm/*/features#macros
[`runtime-tokio-native-tls`]: https://docs.rs/crate/sea-orm/*/features#runtime-tokio-native-tls
[`sqlx-sqlite`]: https://docs.rs/crate/sea-orm/*/features#sqlx-sqlite
[`macros`]: https://docs.rs/crate/tokio/*/features#macros
[`rt-multi-thread`]: https://docs.rs/crate/tokio/*/features#rt-multi-thread
