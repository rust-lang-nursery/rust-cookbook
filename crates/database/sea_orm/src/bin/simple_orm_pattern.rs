use sea_orm::sea_query::TableCreateStatement;
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, DbErr,
    EntityTrait, Schema, Set,
};

mod location {
    use sea_orm::entity::prelude::*;

    #[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "location")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub name: String,
        pub latitude: f64,
        pub longitude: f64,
    }

    #[derive(Debug, Clone, PartialEq, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = Database::connect("sqlite::memory:").await?;

    create_location_table(&db).await?;

    let mut null_island = location::ActiveModel {
        name: Set("Null Island".to_owned()),
        latitude: Set(0.0),
        longitude: Set(0.0),
        ..Default::default()
    }
    .save(&db)
    .await?;

    let location = location::Entity::find().one(&db).await?;
    println!("{location:?}");

    null_island.name = Set("Null Island V2".to_owned());
    let null_island = null_island.save(&db).await?;

    println!("Updated location: {null_island:?}");
    Ok(())
}

async fn create_location_table(db: &DatabaseConnection) -> Result<(), DbErr> {
    let builder = DatabaseBackend::Sqlite;
    let schema = Schema::new(builder);

    let statement: TableCreateStatement = schema.create_table_from_entity(location::Entity);
    db.execute(builder.build(&statement)).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{create_location_table, location};
    use sea_orm::{ActiveModelTrait, Database, EntityTrait, Set, Unchanged};

    #[tokio::test]
    async fn test_seaorm_insert_and_select() -> Result<(), sea_orm::DbErr> {
        let db = Database::connect("sqlite::memory:").await?;

        create_location_table(&db).await?;

        let mut null_island = location::ActiveModel {
            name: Set("Null Island".to_owned()),
            latitude: Set(0.0),
            longitude: Set(0.0),
            ..Default::default()
        }
        .save(&db)
        .await?;

        let location = location::Entity::find().one(&db).await?;

        assert_eq!(
            location,
            Some(location::Model {
                id: 1,
                name: "Null Island".to_owned(),
                latitude: 0.0,
                longitude: 0.0,
            })
        );

        null_island.name = Set("Null Island V2".to_owned());
        let null_island = null_island.save(&db).await?;

        assert_eq!(null_island.name, Unchanged("Null Island V2".to_owned()));

        Ok(())
    }
}
