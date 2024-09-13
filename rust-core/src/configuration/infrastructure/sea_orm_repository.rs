use sea_orm::sea_query::OnConflict;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use shaku::Provider;
use std::error::Error;
use std::sync::Arc;

use crate::configuration::domain::{ConfigurationRepository, NewConfigurationModel};
use crate::entities::configuration;
use crate::infrastructure::sea_orm::connection_provider::ConnectionProvider;

#[derive(Provider)]
#[shaku(interface = ConfigurationRepository)]
pub struct SeaOrmConfigurationRepository {
    #[shaku(inject)]
    connection: Arc<dyn ConnectionProvider>,
}

impl SeaOrmConfigurationRepository {
    pub fn new(connection: Arc<dyn ConnectionProvider>) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl ConfigurationRepository for SeaOrmConfigurationRepository {
    async fn get_all(&self) -> Result<Vec<configuration::Model>, Box<dyn Error>> {
        let conn = self.connection.get();
        let r = configuration::Entity::find().all(conn.as_ref()).await?;

        Ok(r)
    }

    async fn find(&self, id: i32) -> Result<Option<configuration::Model>, Box<dyn Error>> {
        let conn = self.connection.get();
        let r = configuration::Entity::find_by_id(id)
            .one(conn.as_ref())
            .await?;

        Ok(r)
    }

    async fn find_by_key(&self, key: &str) -> Result<Option<configuration::Model>, Box<dyn Error>> {
        let conn = self.connection.get();
        let r = configuration::Entity::find()
            .filter(configuration::Column::Key.eq(key))
            .one(conn.as_ref())
            .await?;

        Ok(r)
    }

    async fn upsert(
        &self,
        item: NewConfigurationModel,
    ) -> Result<configuration::Model, Box<dyn Error>> {
        let conn = self.connection.get();
        let model = configuration::ActiveModel {
            key: Set(item.key),
            value: Set(item.value),
            ..Default::default()
        };

        let on_conflict = OnConflict::column(configuration::Column::Key)
            .update_column(configuration::Column::Value)
            .to_owned();

        let r = configuration::Entity::insert(model)
            .on_conflict(on_conflict)
            .exec_with_returning(conn.as_ref())
            .await?;

        Ok(r)
    }
}
