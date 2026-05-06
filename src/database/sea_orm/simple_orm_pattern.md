## Create, query, and update rows with SeaORM
[![sea_orm-badge]][sea_orm] [![cat-database-badge]][cat-database]

Use [`Database::connect`] to open an in-memory SQLite database and
[`Schema::create_table_from_entity`] to create a table from a SeaORM entity.
Create a row with [`ActiveModelTrait::save`], load it with [`EntityTrait::find`],
then update it and check the saved value returned by [`ActiveModelTrait::save`].

```rust
{{#include ../../../crates/database/sea_orm/src/bin/simple_orm_pattern.rs::58 }}
```

[`Database::connect`]: https://docs.rs/sea_orm/*/sea_orm/struct.Database.html#method.connect
[`Schema::create_table_from_entity`]: https://docs.rs/sea_orm/*/sea_orm/schema/struct.Schema.html#method.create_table_from_entity
[`ActiveModelTrait::save`]: https://docs.rs/sea_orm/*/sea_orm/entity/trait.ActiveModelTrait.html#method.save
[`EntityTrait::find`]: https://docs.rs/sea_orm/*/sea_orm/entity/trait.EntityTrait.html#tymethod.find
