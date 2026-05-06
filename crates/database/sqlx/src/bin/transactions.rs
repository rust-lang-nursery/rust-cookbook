use sqlx::{Error, postgres::PgPoolOptions};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect("postgresql://postgres:postgres@localhost/locations")
        .await?;

    let mut tx = pool.begin().await?;

    sqlx::query("INSERT INTO location (name, latitude, longitude) VALUES ($1, $2, $3)")
        .bind("Area 51")
        .bind(37.2350)
        .bind(-115.8111)
        .execute(&mut *tx)
        .await?;

    sqlx::query("INSERT INTO location (name, latitude, longitude) VALUES ($1, $2, $3)")
        .bind("Point Nemo")
        .bind(-48.8767)
        .bind(-123.3933)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}
