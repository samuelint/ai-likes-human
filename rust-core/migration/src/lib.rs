use sea_orm::DatabaseConnection;
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_configuration_table;
mod m20220101_000002_create_message_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_configuration_table::Migration),
            Box::new(m20220101_000002_create_message_table::Migration),
        ]
    }
}

pub async fn migrate_database(db: &DatabaseConnection) -> Result<(), DbErr> {
    let schema_manager = SchemaManager::new(db);

    Migrator::up(db, None).await?;
    assert!(schema_manager.has_table("configuration").await?);

    Ok(())
}
