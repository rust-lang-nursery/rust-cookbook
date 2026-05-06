use sqlx::{Connection, Error, postgres::PgConnection};

struct Location {
    name: String,
    latitude: f64,
    longitude: f64,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut conn =
        PgConnection::connect("postgresql://postgres:postgres@localhost/locations").await?;

    let location = sqlx::query_as!(
        Location,
        "SELECT  name, latitude, longitude FROM location where id = $1",
        1i32
    )
    .fetch_one(&mut conn)
    .await?;

    println!(
        "{} is at coordinates latitude: {} and longitude: {}",
        location.name, location.latitude, location.longitude
    );

    Ok(())
}
