## Posgresデータベースにテーブルを作る

[![postgres-badge]][postgres] [![cat-database-badge]][cat-database]

[`postgres`]クレートを使ってPostgresデータベースにテーブルを作る。

[`Connection::connect`]で既存のデータベースに接続する。このレシピは`Connection::connect`のURL文字列フォーマットを使っています。既存のデータベース名は`library`と仮定し、ユーザーネームは`postgres`、パスワードは`postgres`とする。


```rust,no_run
extern crate postgres;

use postgres::{Connection, TlsMode, Error};

fn main() -> Result<(), Error> {
    let conn = Connection::connect("postgresql://postgres:postgres@localhost/library", 
                                    TlsMode::None)?;
    
     conn.execute("CREATE TABLE IF NOT EXISTS author (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    country         VARCHAR NOT NULL
                  )", &[])?;

    conn.execute("CREATE TABLE IF NOT EXISTS book  (
                    id              SERIAL PRIMARY KEY,
                    title           VARCHAR NOT NULL,
                    author_id       INTEGER NOT NULL REFERENCES author
                )", &[])?;

    Ok(())

}
```

[`postgres`]: https://docs.rs/postgres/0.15.2/postgres/
[`Connection::connect`]: https://docs.rs/postgres/0.15.2/postgres/struct.Connection.html#method.connect
