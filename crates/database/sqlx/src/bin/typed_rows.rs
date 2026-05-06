use sqlx::{Error, postgres::PgPool};

#[derive(sqlx::FromRow)]
struct Location {
    name: String,
    latitude: f64,
    longitude: f64,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let pool =
        PgPool::connect("postgresql://postgres:postgres@localhost/locations").await?;

    let locations = sqlx::query_as::<_, Location>("SELECT name, latitude, longitude FROM location")
        .fetch_all(&pool)
        .await?;

    for location in locations {
        println!(
            "{} is at coordinates latitude: {} and longitude: {}",
            location.name, location.latitude, location.longitude
        );
    }

    Ok(())
}
