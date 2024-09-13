use migration::migrate_database;
use std::{error::Error, sync::Arc};

use sea_orm::{Database, DatabaseConnection};

pub struct ConnectionFactory {
    database_url: String,
}

impl ConnectionFactory {
    pub fn new(database_url: String) -> Self {
        Self { database_url }
    }

    pub async fn create(&self) -> Result<Arc<DatabaseConnection>, Box<dyn Error>> {
        let db: DatabaseConnection = Database::connect(self.database_url.as_str()).await?;
        migrate_database(&db).await?;

        Ok(Arc::new(db))
    }
}
