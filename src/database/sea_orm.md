# SeaORM

[`SeaORM`][sea_orm] maps database tables to Rust entities and active models, so
you can create, query, and update rows with typed values instead of writing raw
SQL for each step. The entity in the recipe below shows the generated Rust
types clearly, but you do not need to write all of that code by hand when you
already have a database schema. Use [`sea-orm-cli`] to generate entities from
an existing database.

{{#include sea_orm/simple_orm_pattern.md}}

{{#include ../links.md}}

[`sea-orm-cli`]: https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/
