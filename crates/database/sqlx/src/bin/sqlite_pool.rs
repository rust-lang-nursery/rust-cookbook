use sqlx::{Error, Row, sqlite::SqlitePoolOptions};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS location (
            id                  INTEGER PRIMARY KEY,
            name                TEXT NOT NULL,
            latitude            REAL NOT NULL,
            longitude           REAL NOT NULL
            )
        ",
    )
    .execute(&pool)
    .await?;

    sqlx::query("INSERT INTO location (name, latitude, longitude) VALUES (?1, ?2, ?3)")
        .bind("Null Island")
        .bind(0.00)
        .bind(0.00)
        .execute(&pool)
        .await?;

    sqlx::query("INSERT INTO location (name, latitude, longitude) VALUES (?1, ?2, ?3)")
        .bind("Titanic")
        .bind(41.726931)
        .bind(-49.948253)
        .execute(&pool)
        .await?;

    let locations = sqlx::query("SELECT name, latitude, longitude FROM location")
        .fetch_all(&pool)
        .await?;

    for row in locations {
        let name: String = row.try_get("name")?;
        let lat: f64 = row.try_get("latitude")?;
        let longitude: f64 = row.try_get("longitude")?;
        println!("{name} is at coordinates latitude: {lat} and longitude: {longitude}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use sqlx::Row;
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn test_sqlite_pool_insert_and_select() -> Result<(), sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS location (
            id                  INTEGER PRIMARY KEY,
            name                TEXT NOT NULL,
            latitude            REAL NOT NULL,
            longitude           REAL NOT NULL
            )
        ",
        )
        .execute(&pool)
        .await?;

        sqlx::query("INSERT INTO location (name, latitude, longitude) VALUES (?1, ?2, ?3)")
            .bind("Null Island")
            .bind(0.00)
            .bind(0.00)
            .execute(&pool)
            .await?;

        let locations = sqlx::query("SELECT name, latitude, longitude FROM location")
            .fetch_all(&pool)
            .await?;

        assert_eq!(locations.len(), 1);
        let name: String = locations[0].try_get("name")?;
        assert_eq!(name, "Null Island");
        Ok(())
    }
}
