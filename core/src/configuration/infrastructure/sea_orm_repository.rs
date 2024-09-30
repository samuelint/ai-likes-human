use crate::configuration::domain::dto::ConfigurationDto;
use crate::configuration::domain::ConfigurationRepository;
use crate::entities::configuration;
use anyhow::anyhow;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::error::Error;
use std::sync::Arc;

pub struct SeaOrmConfigurationRepository {
    connection: Arc<DatabaseConnection>,
}

impl SeaOrmConfigurationRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl ConfigurationRepository for SeaOrmConfigurationRepository {
    async fn find(&self, id: i32) -> Result<Option<ConfigurationDto>, Box<dyn Error + Send>> {
        let conn = Arc::clone(&self.connection);
        let configuration = configuration::Entity::find_by_id(id)
            .one(conn.as_ref())
            .await
            .map_err(|e| anyhow!(e))?;

        if configuration.is_none() {
            return Ok(None);
        }

        let configuration: ConfigurationDto = (&configuration.unwrap()).into();

        Ok(Some(configuration))
    }

    async fn find_by_key(
        &self,
        key: &str,
    ) -> Result<Option<ConfigurationDto>, Box<dyn Error + Send>> {
        let conn = Arc::clone(&self.connection);
        let configuration = configuration::Entity::find()
            .filter(configuration::Column::Key.eq(key))
            .one(conn.as_ref())
            .await
            .map_err(|e| anyhow!(e))?;

        if configuration.is_none() {
            return Ok(None);
        }

        let configuration: ConfigurationDto = (&configuration.unwrap()).into();

        Ok(Some(configuration))
    }

    async fn upsert(
        &self,
        item: ConfigurationDto,
    ) -> Result<ConfigurationDto, Box<dyn Error + Send>> {
        let conn = Arc::clone(&self.connection);
        let model = configuration::ActiveModel {
            key: Set(item.key.clone()),
            value: Set(item.value),
            ..Default::default()
        };

        let on_conflict = OnConflict::column(configuration::Column::Key)
            .update_column(configuration::Column::Value)
            .to_owned();

        configuration::Entity::insert(model)
            .on_conflict(on_conflict)
            .exec(conn.as_ref())
            .await
            .map_err(|e| anyhow!(e))?;

        let result = configuration::Entity::find()
            .filter(configuration::Column::Key.eq(item.key))
            .one(conn.as_ref())
            .await
            .map_err(|e| anyhow!(e))?
            .ok_or_else(|| "Failed to find inserted item")
            .map_err(|e| anyhow!(e))?;

        Ok((&result).into())
    }
}
