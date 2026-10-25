## Using transactions

[![rusqlite-badge]][rusqlite] [![cat-database-badge]][cat-database]

[`Connection::open`] will open the `cats.db` database from the top recipe.

Begin a transaction with [`Connection::transaction`]. Transactions will
roll back unless committed explicitly with [`Transaction::commit`].

In the following recipe, colors add to a table having
a unique constraint on the color name. When an attempt to insert
a duplicate color is made, the transaction rolls back.


```rust,no_run
{{#include ../../../crates/database/rusqlite/src/bin/transactions.rs::33 }}
```

[`Connection::transaction`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.transaction
[`Transaction::commit`]: https://docs.rs/rusqlite/*/rusqlite/struct.Transaction.html#method.commit
